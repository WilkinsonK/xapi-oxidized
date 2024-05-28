use std::{fmt::Debug, rc::Rc};

use oxinat_derive::uri_builder_alias;

use crate::{UriBuildError, UriBuilder, Version};

uri_builder_alias!(EventsAdminUriBuilder);
ImplEventsAdminUriBuilder! {
    (String),
}

/// Represents the URI paths available for
/// endpoints meant for managing events.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/events")]
#[match_path(path = "{parent}/projects/{project}/events")]
pub struct EventsUriBuilder<Parent>
where
    Parent: EventsAdminUriBuilder,
{
    #[param]
    project: Option<String>,
    #[parent]
    parent: Option<Rc<Parent>>
}

/// Dictates what the subendpoint should be for
/// when building URIs for some type of action.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum EventType {
    #[default]
    All,
    ByEvent,
    Multiple,
    One,
}

/// Generates a predicate usable to confirm if
/// the event type of some builder matches the
/// declared type.
macro_rules! is_event_type {
    () => {
        (|this: &Self| this.event_type == EventType::All)
    };
    ($type:ident) => {
        (|this: &Self| this.event_type == EventType::$type)
    };
}

/// Represents URI endpoints for action related
/// events.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/action", requires = "is_event_type!(One)")]
#[match_path(path = "{parent}/actions", requires = "is_event_type!(Multiple)")]
#[match_path(path = "{parent}/actionsbyevent", requires = "is_event_type!(ByEvent)")]
#[match_path(path = "{parent}/allactions", requires = "is_event_type!()")]
pub struct ActionsUriBuilder<'a> {
    event_type: EventType,
    #[parent]
    parent: Option<&'a EventsUriBuilder<String>>
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DeliveredType {
    CleanUp,
    Count,
    Summary,
    #[default]
    None
}

/// Generates a predicate usable to confirm if
/// the delivered type of some builder matches the
/// declared type.
macro_rules! is_deliver_type {
    () => {
        (|this: &Self| this.deliver_type == DeliveredType::default())
    };
    ($type:ident) => {
        (|this: &Self| this.deliver_type == DeliveredType::$type)
    };
}

/// Represents URI endpoints available to manage
/// events related to delivered subscriptions.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/delivered/cleanup", requires = "is_deliver_type!(CleanUp)")]
#[match_path(path = "{parent}/delivered/count", requires = "is_deliver_type!(Count)")]
#[match_path(path = "{parent}/delivered/summary", requires = "is_deliver_type!(Summary)")]
#[match_path(path = "{parent}/delivered/{id}")]
#[match_path(path = "{parent}/delivered")]
pub struct DeliveredUriBuilder<'a> {
    deliver_type: DeliveredType,
    #[param]
    id: Option<String>,
    #[parent]
    parent: Option<&'a EventsUriBuilder<String>>
}

/// Represents URI endpoints available for
/// management of events.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/event", requires = "is_event_type!(One)")]
#[match_path(path = "{parent}/events")]
pub struct EventUriBuilder<'a> {
    event_type: EventType,
    #[parent]
    parent: Option<&'a EventsUriBuilder<String>>
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SubscriptionAction {
    Activate,
    All,
    Deactivate,
    Filter,
    Validate,
    #[default]
    None
}

/// Generates a predicate usable to confirm if
/// the subscription action of some builder
/// matches the declared type.
macro_rules! is_subscription_action {
    () => {
        (|this: &Self| this.action == SubscriptionAction::default())
    };
    ($type:ident) => {
        (|this: &Self| this.action == SubscriptionAction::$type)
    };
}

/// Represents URI endpoints available for
/// management of subsription events.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/subscriptions", requires = "is_subscription_action!(All)")]
#[match_path(path = "{parent}/subscription/filter", requires = "is_subscription_action!(Filter)")]
#[match_path(path = "{parent}/subscription/validate", requires = "is_subscription_action!(Validate)")]
#[match_path(path = "{parent}/subscription/{id}/activate", requires = "is_subscription_action!(Activate)")]
#[match_path(path = "{parent}/subscription/{id}/deactivate", requires = "is_subscription_action!(Deactivate)")]
#[match_path(path = "{parent}/subscription/{id}")]
#[match_path(path = "{parent}/subscription")]
pub struct SubscriptionUriBuilder<'a> {
    action: SubscriptionAction,
    #[param]
    id: Option<String>,
    #[parent]
    parent: Option<&'a EventsUriBuilder<String>>,
}

impl EventUriBuilder<'_> {
    /// Produce the event/properties URI endpoint.
    pub fn build_properties(&self) -> anyhow::Result<String> {
        if is_event_type!(One)(self) {
            self.build_join("properties")
        } else {
            Err(UriBuildError::Validation.into())
        }
    }
}

impl EventsUriBuilder<String> {
    /// Continue the builder into a
    /// `ActionsUriBuilder`.
    pub fn actions(&self) -> ActionsUriBuilder {
        ActionsUriBuilder::from_parent(&Rc::new(self))
    }

    /// Produces the events/prefs URI
    /// endpoint.
    pub fn build_pref(&self) -> anyhow::Result<String> {
        self.build_join("prefs")
    }

    /// Continue the builder into a
    /// `DeliveredUriBuilder`.
    pub fn delivered(&self) -> DeliveredUriBuilder {
        DeliveredUriBuilder::from_parent(&Rc::new(self))
    }

    /// Continue the builder into a
    /// `EventUriBuilder`.
    pub fn event(&self) -> EventUriBuilder {
        EventUriBuilder::from_parent(&Rc::new(self))
    }

    /// Continue the builder into a
    /// `SubscriptionUriBuilder`.
    pub fn subscription(&self) -> SubscriptionUriBuilder {
        SubscriptionUriBuilder::from_parent(&Rc::new(self))
    }
}

/// Represents URI paths for event tracking API.
#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/event_tracking")]
#[match_path(path = "{parent}/event_tracking/{key}")]
pub struct EventTrackingUriBuilder<Parent>
where
    Parent: EventsAdminUriBuilder,
{
    #[param]
    key: Option<String>,
    #[parent]
    parent: Option<Rc<Parent>>,
}

/// Represents the URI paths available for
/// endpoints meant for managing events.
pub trait EventsUri: Version {
    /// URI endpoints to manage XNAT events.
    #[inline]
    fn events(&self) -> EventsUriBuilder<String> {
        EventsUriBuilder::from_parent(self.root_uri().into())
    }

    /// URI endpoints to manage XNAT event
    /// tracking.
    #[inline]
    fn event_tracking(&self) -> EventTrackingUriBuilder<String> {
        EventTrackingUriBuilder::from_parent(self.root_uri().into())
    }
}
