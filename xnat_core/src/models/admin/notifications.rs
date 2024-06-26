//! Generated via `helpers/gen_models.py`
//! Do not edit this file directly.
//! 
//! Defines models and model units available to a
//! user for manipulating `./notifications` values.

use serde::{Deserialize, Serialize};

use oxinat_derive::ModelField;

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "copyAdminOnNotifications")]
pub struct CopyAdminOnNotifications(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "copyAdminOnPageEmails")]
pub struct CopyAdminOnPageEmails(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailAllowNonuserSubscribers")]
pub struct EmailAllowNonuserSubscribers(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailMessageForgotPasswordReset")]
pub struct EmailMessageForgotPasswordReset(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailMessageForgotUsernameRequest")]
pub struct EmailMessageForgotUsernameRequest(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailMessageUserRegistration")]
pub struct EmailMessageUserRegistration(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailPrefix")]
pub struct EmailPrefix(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailRecipientErrorMessages")]
pub struct EmailRecipientErrorMessages(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailRecipientIssueReports")]
pub struct EmailRecipientIssueReports(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailRecipientNewUserAlert")]
pub struct EmailRecipientNewUserAlert(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailRecipientUpdate")]
pub struct EmailRecipientUpdate(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "helpContactInfo")]
pub struct HelpContactInfo(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "notifyAdminPipelineEmails")]
pub struct NotifyAdminPipelineEmails(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "notifyAdminProjectAccessRequest")]
pub struct NotifyAdminProjectAccessRequest(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "notifyAdminSessionTransfer")]
pub struct NotifyAdminSessionTransfer(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "notifyAdminUserRegistration")]
pub struct NotifyAdminUserRegistration(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtpAuth")]
pub struct SmtpAuth(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtpEnabled")]
pub struct SmtpEnabled(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtpHostname")]
pub struct SmtpHostname(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtpPassword")]
pub struct SmtpPassword(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtpPort")]
pub struct SmtpPort(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtpProtocol")]
pub struct SmtpProtocol(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtpSslTrust")]
pub struct SmtpSslTrust(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtpStartTls")]
pub struct SmtpStartTls(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtpUsername")]
pub struct SmtpUsername(String);

#[derive(Debug, Deserialize, Serialize)]
pub struct Notifications {
    #[serde(rename = "copyAdminOnNotifications")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_admin_on_notifications: Option<CopyAdminOnNotifications>,
    #[serde(rename = "copyAdminOnPageEmails")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_admin_on_page_emails: Option<CopyAdminOnPageEmails>,
    #[serde(rename = "emailAllowNonuserSubscribers")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_allow_nonuser_subscribers: Option<EmailAllowNonuserSubscribers>,
    #[serde(rename = "emailMessageForgotPasswordReset")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message_forgot_password_reset: Option<EmailMessageForgotPasswordReset>,
    #[serde(rename = "emailMessageForgotUsernameRequest")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message_forgot_username_request: Option<EmailMessageForgotUsernameRequest>,
    #[serde(rename = "emailMessageUserRegistration")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message_user_registration: Option<EmailMessageUserRegistration>,
    #[serde(rename = "emailPrefix")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_prefix: Option<EmailPrefix>,
    #[serde(rename = "emailRecipientErrorMessages")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_recipient_error_messages: Option<EmailRecipientErrorMessages>,
    #[serde(rename = "emailRecipientIssueReports")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_recipient_issue_reports: Option<EmailRecipientIssueReports>,
    #[serde(rename = "emailRecipientNewUserAlert")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_recipient_new_user_alert: Option<EmailRecipientNewUserAlert>,
    #[serde(rename = "emailRecipientUpdate")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_recipient_update: Option<EmailRecipientUpdate>,
    #[serde(rename = "helpContactInfo")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub help_contact_info: Option<HelpContactInfo>,
    #[serde(rename = "notifyAdminPipelineEmails")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_admin_pipeline_emails: Option<NotifyAdminPipelineEmails>,
    #[serde(rename = "notifyAdminProjectAccessRequest")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_admin_project_access_request: Option<NotifyAdminProjectAccessRequest>,
    #[serde(rename = "notifyAdminSessionTransfer")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_admin_session_transfer: Option<NotifyAdminSessionTransfer>,
    #[serde(rename = "notifyAdminUserRegistration")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_admin_user_registration: Option<NotifyAdminUserRegistration>,
    #[serde(rename = "smtpAuth")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_auth: Option<SmtpAuth>,
    #[serde(rename = "smtpEnabled")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_enabled: Option<SmtpEnabled>,
    #[serde(rename = "smtpHostname")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_hostname: Option<SmtpHostname>,
    #[serde(rename = "smtpPassword")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_password: Option<SmtpPassword>,
    #[serde(rename = "smtpPort")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_port: Option<SmtpPort>,
    #[serde(rename = "smtpProtocol")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_protocol: Option<SmtpProtocol>,
    #[serde(rename = "smtpSslTrust")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_ssl_trust: Option<SmtpSslTrust>,
    #[serde(rename = "smtpStartTls")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_start_tls: Option<SmtpStartTls>,
    #[serde(rename = "smtpUsername")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_username: Option<SmtpUsername>,
}

