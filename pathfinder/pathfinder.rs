use std::collections::HashMap;

struct SearchTracker {
    counts: HashMap<String, uint>
}
impl SearchTracker {
    fn new<'a, I: Iterator<&'a String>>(mut service_names: I) -> SearchTracker {
        let mut tracker = SearchTracker{counts: HashMap::new()};
        for name in service_names {
            tracker.counts.insert(name.clone(), 0);
        }
        tracker
    }

    fn add_search(&mut self) {
        for (_, count) in self.counts.mut_iter() {
            *count += 1;
        }
    }

    fn mark_done(&mut self, name: String) {
        let count = self.counts.get_mut(&name);
        *count -= 1;
    }

    fn is_done(&self) -> bool {
        self.counts.values().all(|n| *n == 0)
    }
}

struct MultiTaskSender<T> {
    senders: Vec<Sender<T>>
}
impl<T: Clone + Send> MultiTaskSender<T> {
    fn new() -> MultiTaskSender<T> {
        MultiTaskSender{senders: vec![]}
    }

    fn add_sender(&mut self, sender: Sender<T>) {
        self.senders.push(sender);
    }

    fn send(&self, t: T) {
        for sender in self.senders.iter() {
            sender.send(t.clone());
        }
    }
}

#[deriving(Clone)]
struct Path {
    from:    String,
    to:      String,
    service: String
}
impl Path {
    fn new(from: String, to: String, service: String) -> Path {
        Path{from: from, to: to, service: service}
    }

    fn to_string(&self) -> String {
        self.from
            .clone()
            .append("--")
            .append(self.service.as_slice())
            .append("-->")
            .append(self.to.as_slice())
    }
}

#[deriving(Clone)]
struct Search {
    from:  String,
    to:    String,
    paths: Vec<Path>
}
impl Search {
    fn new(from: String, to: String) -> Search {
        Search{from: from, to: to, paths: vec![]}
    }

    fn services(&self) -> Vec<String> {
        self.paths.iter().map(|path| path.service.clone()).collect()
    }

    fn stops(&self) -> Vec<String> {
        let mut all = vec![];
        all.push(self.from.clone());
        all.push(self.to.clone());
        for path in self.paths.iter() {
            all.push(path.from.clone());
            all.push(path.to.clone());
        }
        all
    }

    fn add_path(&self, path: Path) -> Search {
        Search{ from:  path.to.clone(),
                to:    self.to.clone(),
                paths: self.paths.clone().append([path]) }
    }
}

enum Event {
    Match(Vec<Path>),
    Partial(Vec<Search>),
    Done(String)
}

#[deriving(Clone)]
enum Job {
    Work(Search),
    Finish
}

struct TaskManager {
    services:       HashMap<String, Vec<String>>,
    tracker:        SearchTracker,
    multi_sender:   MultiTaskSender<Job>,
    event_sender:   Sender<Event>,
    event_receiver: Receiver<Event>
}
impl TaskManager {
    fn new(services: HashMap<String, Vec<String>>) -> TaskManager {
        let (sender, receiver) = channel();
        TaskManager{ services:       services.clone(),
                     tracker:        SearchTracker::new(services.keys()),
                     multi_sender:   MultiTaskSender::new(),
                     event_sender:   sender,
                     event_receiver: receiver }
    }

    fn run(&mut self, work: Job) {
        self.launch_services();
        self.send_job(work);
        self.wait_for_services();
        self.send_job(Finish);
    }

    fn launch_services(&mut self) {
        for (name, stops) in self.services.clone().move_iter() {
            let task_event_sender                = self.event_sender.clone();
            let (search_sender, search_receiver) = channel();
            self.multi_sender.add_sender(search_sender.clone());
            spawn( proc() {
                loop {
                    let job = search_receiver.recv();
                    match job {
                        Work(search) => {
                            if stops.contains(&search.from) &&
                               stops.contains(&search.to) {
                                let path  = Path::new(
                                    search.from,
                                    search.to,
                                    name.clone()
                                );
                                let paths = search.paths.append([path]);
                                task_event_sender.send(Match(paths))
                            } else {
                                let mut tos      = stops.clone();
                                let     previous = search.stops();
                                tos.retain(|stop| !previous.contains(stop));
                                if !search.services().contains(&name) &&
                                   stops.contains(&search.from)       &&
                                   !tos.is_empty() {
                                    let searches = tos.iter().map( |to| {
                                        let path = Path::new(
                                            search.from.clone(),
                                            to.clone(),
                                            name.clone()
                                        );
                                        search.add_path(path)
                                    } ).collect();
                                    task_event_sender.send(Partial(searches));
                                } else {
                                    task_event_sender.send(Done(name.clone()));
                                }
                            }
                        }
                        Finish       => { break; }
                    }
                }
            } );
        }
    }

    fn wait_for_services(&mut self) {
        loop {
            match self.event_receiver.recv() {
                Match(paths)      => {
                    let name = paths.last().expect("No path").service.clone();
                    self.tracker.mark_done(name);

                    let path_string = paths.iter().skip(1).fold(
                        paths[0].to_string(),
                        |s, p| s.append(p.to_string().as_slice().slice_from(1))
                    );
                    println!("Path: {}", path_string);
                }
                Partial(searches) => {
                    let name = searches.last()
                                       .expect("No search")
                                       .paths
                                       .last()
                                       .expect("No path")
                                       .service
                                       .clone();
                    self.tracker.mark_done(name);

                    for search in searches.iter() {
                        self.send_job(Work(search.clone()));
                    }
                }
                Done(name)        => { self.tracker.mark_done(name) }
            }
            if self.tracker.is_done() { break; }
        }
    }

    fn send_job(&mut self, job: Job) {
        match job {
            Work(_) => { self.tracker.add_search(); }
            Finish  => { /* do nothing */ }
        }
        self.multi_sender.send(job);
    }
}

fn string_vec(strs: &[&'static str]) -> Vec<String> {
    let mut v = Vec::new();
    for s in strs.iter() {
        v.push(s.to_string());
    }
    v
}

fn main() {
    let mut services = HashMap::new();
    services.insert("S1".to_string(), string_vec(["A", "B"]));
    services.insert("S2".to_string(), string_vec(["A", "C"]));
    services.insert("S3".to_string(), string_vec(["C", "D", "E", "F"]));
    services.insert("S4".to_string(), string_vec(["D", "B"]));

    let work = Work(Search::new("A".to_string(), "B".to_string()));

    let mut task_manager = TaskManager::new(services);
    task_manager.run(work);
}
