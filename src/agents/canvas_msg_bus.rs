use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    CanvasSelectMsg((i32,i32,i32,i32)),
}

pub struct CanvasMsgBus {
    link: AgentLink<CanvasMsgBus>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for CanvasMsgBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = (i32,i32,i32,i32);

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::CanvasSelectMsg(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
