use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};


/// CommandMsgBus
/// Transmits messages from the control panel component to the canvas component

pub struct CommandMsgBus {
    link: AgentLink<CommandMsgBus>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for CommandMsgBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = CommandRequest;
    type Output = CommandRequest;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommandRequest {
    Start,
    Stop,
    Clear,
    Copy
}

