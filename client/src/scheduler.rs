use std::{
    any::Any,
    fmt,
    future::Future,
    sync::mpsc::{self, Receiver, Sender},
};

pub struct Scheduler {
    sender: Sender<Report>,
    receiver: Receiver<Report>,
}

impl Scheduler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }

    pub fn spawn<F, T>(&self, task: F) -> TaskId
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        use async_std::task::{self, Builder};

        let sender = self.sender.clone();
        let future = async move {
            let value = task.await;
            let report = Report {
                id: TaskId(task::current().id()),
                value: Box::new(value),
            };
            sender.send(report).expect("send");
        };

        let id = Builder::new()
            .spawn(future)
            .expect("cannot spawn task")
            .task()
            .id();

        TaskId(id)
    }

    pub fn ready(&self) -> impl Iterator<Item = Report> + '_ {
        (0..).map_while(|_| self.receiver.try_recv().ok())
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct TaskId(async_std::task::TaskId);

impl fmt::Debug for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TaskId({})", self.0)
    }
}

pub struct Report {
    pub id: TaskId,
    pub value: Box<dyn Any + Send>,
}
