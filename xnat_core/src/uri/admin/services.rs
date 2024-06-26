use std::{fmt::Debug, sync::Arc};

use crate::{UriBuilder, Version};

#[derive(Clone, Debug, Default, UriBuilder)]
#[match_path(path = "{parent}/services")]
pub struct ServicesUriLegacyBuilder<Parent>
where
    Parent: Clone + Debug + UriBuilder,
{
    #[parent]
    parent: Option<Arc<Parent>>
}

impl ServicesUriLegacyBuilder<String> {
    /// Produces the services/audit URI path.
    pub fn build_audit(&self, xsi_type: &str, object_id: &str) -> crate::BuildResult {
        self.build_join(format!("audit/{xsi_type}/{object_id}"))
    }

    /// Produces the services/features URI path.
    pub fn build_features(&self) -> crate::BuildResult {
        self.build_join("features")
    }

    /// Produces the services/refresh/catalog URI
    /// path.
    pub fn build_refresh_catalog(&self) -> crate::BuildResult {
        self.build_join("refresh/catalog")
    }

    /// Produces the services/mail/sent URI path.
    pub fn build_mail_send(&self) -> crate::BuildResult {
        self.build_join("mail/send")
    }

    /// Produces the services/move-files URI path.
    pub fn build_move_files(&self) -> crate::BuildResult {
        self.build_join("move-files")
    }

    /// Produces the
    /// services/sendEmailVerification URI path.
    pub fn build_send_email_verification(&self) -> crate::BuildResult {
        self.build_join("sendEmailVerification")
    }
}

/// Represents the URI paths available for legacy
/// service management.
pub trait ServicesUriLegacy: Version {
    #[inline]
    fn services(&self) -> ServicesUriLegacyBuilder<String> {
        ServicesUriLegacyBuilder::from_parent(self.data_uri().into())
    }
}
