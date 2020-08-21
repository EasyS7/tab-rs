use crate::{type_name::type_name, Storage};

use std::fmt::{Debug, Display};
use thiserror::Error;

pub trait Channel {
    type Tx: Storage + Send + 'static;
    type Rx: Storage + Send + 'static;

    fn channel(capacity: usize) -> (Self::Tx, Self::Rx);

    fn default_capacity() -> usize;

    fn clone_tx(tx: &mut Option<Self::Tx>) -> Option<Self::Tx> {
        Self::Tx::take_or_clone(tx)
    }

    fn clone_rx(rx: &mut Option<Self::Rx>, _tx: Option<&Self::Tx>) -> Option<Self::Rx> {
        Self::Rx::take_or_clone(rx)
    }
}

pub trait Message<Bus>: Debug {
    type Channel: Channel;
}

pub trait Carries<Type> {}
impl<B, T> Carries<T> for B where T: Message<B> {}

pub trait Resource<Bus>: Storage + Debug + Send {}

pub trait Stores<Type> {}
impl<B, R> Stores<R> for B where R: Resource<B> {}

pub trait Bus: Debug + Sized {
    /// Returns the receiver on the first call, and

    fn capacity<Msg>(&self, capacity: usize) -> Result<(), AlreadyLinkedError>
    where
        Msg: Message<Self> + 'static;

    fn rx<Msg>(&self) -> Result<<Msg::Channel as Channel>::Rx, LinkTakenError>
    where
        Msg: Message<Self> + 'static;

    fn tx<Msg>(&self) -> Result<<Msg::Channel as Channel>::Tx, LinkTakenError>
    where
        Msg: Message<Self> + 'static;

    fn resource<Res>(&self) -> Result<Res, TakeResourceError>
    where
        Res: Resource<Self>;
}

// struct Msg;

// struct MyChannelBus {}

// impl Channel<MyChannelBus> for Msg {
//     type Tx = mpsc::Sender<Msg>;
//     type Rx = mpsc::Receiver<Msg>;

//     fn channel() -> (Self::Tx, Self::Rx) {
//         mpsc::channel(1)
//     }
// }

#[derive(Debug)]
pub enum Link {
    Tx,
    Rx,
    Both,
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Link::Tx => f.write_str("Tx"),
            Link::Rx => f.write_str("Rx"),
            Link::Both => f.write_str("Both"),
        }
    }
}

#[derive(Error, Debug)]
pub enum TakeChannelError {
    #[error("channel already linked: {0}")]
    AlreadyLinked(AlreadyLinkedError),
    #[error("channel already taken: {0}")]
    AlreadyTaken(LinkTakenError),
}

impl TakeChannelError {
    pub fn already_linked<Bus, Msg>() -> Self {
        Self::AlreadyLinked(AlreadyLinkedError::new::<Bus, Msg>())
    }

    pub fn already_taken<Bus, Msg>(link: Link) -> Self {
        Self::AlreadyTaken(LinkTakenError::new::<Bus, Msg>(link))
    }
}

//TODO: encode Bus and Link types
#[derive(Error, Debug)]
#[error("link already taken: {bus} < {message}::{link} >")]
pub struct LinkTakenError {
    pub bus: String,
    pub message: String,
    pub link: Link,
}

impl LinkTakenError {
    pub fn new<Bus, Message>(link: Link) -> Self {
        LinkTakenError {
            bus: type_name::<Bus>().to_string(),
            message: type_name::<Message>().to_string(),
            link,
        }
    }
}

#[derive(Error, Debug)]
#[error("link already generated: {bus} < {message} >")]
pub struct AlreadyLinkedError {
    pub bus: String,
    pub message: String,
}

impl AlreadyLinkedError {
    pub fn new<Bus, Message>() -> Self {
        AlreadyLinkedError {
            bus: type_name::<Bus>().to_string(),
            message: type_name::<Message>().to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum TakeResourceError {
    #[error("{0}")]
    Uninitialized(ResourceUninitializedError),
    #[error("{0}")]
    Taken(ResourceTakenError),
}

impl TakeResourceError {
    pub fn uninitialized<Bus, Res>() -> Self {
        Self::Uninitialized(ResourceUninitializedError::new::<Bus, Res>())
    }

    pub fn taken<Bus, Res>() -> Self {
        Self::Taken(ResourceTakenError::new::<Bus, Res>())
    }
}

#[derive(Error, Debug)]
#[error("resource already taken: {bus} < {resource} >")]
pub struct ResourceTakenError {
    pub bus: String,
    pub resource: String,
}

impl ResourceTakenError {
    pub fn new<Bus, Res>() -> Self {
        ResourceTakenError {
            bus: type_name::<Bus>().to_string(),
            resource: type_name::<Res>().to_string(),
        }
    }
}

#[derive(Error, Debug)]
#[error("resource uninitialized: {bus} < {resource} >")]
pub struct ResourceUninitializedError {
    pub bus: String,
    pub resource: String,
}

impl ResourceUninitializedError {
    pub fn new<Bus, Res>() -> Self {
        ResourceUninitializedError {
            bus: type_name::<Bus>().to_string(),
            resource: type_name::<Res>().to_string(),
        }
    }
}

#[derive(Error, Debug)]
#[error("resource already initialized: {bus} < {resource} >")]
pub struct ResourceInitializedError {
    pub bus: String,
    pub resource: String,
}

impl ResourceInitializedError {
    pub fn new<Bus, Res>() -> Self {
        ResourceInitializedError {
            bus: type_name::<Bus>().to_string(),
            resource: type_name::<Res>().to_string(),
        }
    }
}