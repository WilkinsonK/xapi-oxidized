//! Generated via `helpers/gen_models.py`
//! Do not edit this file directly.
//! 
//! Defines models and model units available to a
//! user for manipulating `./siteConfig` values.

use serde::{Deserialize, Serialize};

use oxinat_derive::ModelField;

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "adminEmail")]
pub struct AdminEmail(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "aliasTokenTimeout")]
pub struct AliasTokenTimeout(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "aliasTokenTimeoutSchedule")]
pub struct AliasTokenTimeoutSchedule(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "allowDataAdmins")]
pub struct AllowDataAdmins(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "allowHtmlResourceRendering")]
pub struct AllowHtmlResourceRendering(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "allowNonAdminsToClaimUnassignedSessions")]
pub struct AllowNonAdminsToClaimUnassignedSessions(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "archivePath")]
pub struct ArchivePath(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "buildPath")]
pub struct BuildPath(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "cachePath")]
pub struct CachePath(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "canResetFailedLoginsWithForgotPassword")]
pub struct CanResetFailedLoginsWithForgotPassword(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "checksums")]
pub struct Checksums(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "concurrentMaxSessions")]
pub struct ConcurrentMaxSessions(u64);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "csrfEmailAlert")]
pub struct CsrfEmailAlert(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "dataPaths")]
pub struct DataPaths(Vec<String>);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "emailVerification")]
pub struct EmailVerification(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "emailVerificationMessage")]
pub struct EmailVerificationMessage(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "enableCsrfToken")]
pub struct EnableCsrfToken(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "enableDicomReceiver")]
pub struct EnableDicomReceiver(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "enableDicomReceiverPropertyChangedListener")]
pub struct EnableDicomReceiverPropertyChangedListener(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "enableProjectAppletScript")]
pub struct EnableProjectAppletScript(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "enableSitewideAnonymizationScript")]
pub struct EnableSitewideAnonymizationScript(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "enableSitewideSeriesImportFilter")]
pub struct EnableSitewideSeriesImportFilter(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "failMergeOn")]
pub struct FailMergeOn(Vec<String>);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "featureRepositoryService")]
pub struct FeatureRepositoryService(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "featureService")]
pub struct FeatureService(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "ftpPath")]
pub struct FtpPath(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "imageSessionDisplayNamePlural")]
pub struct ImageSessionDisplayNamePlural(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "imageSessionDisplayNameSingular")]
pub struct ImageSessionDisplayNameSingular(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "inactivityBeforeLockout")]
pub struct InactivityBeforeLockout(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "inactivityBeforeLockoutSchedule")]
pub struct InactivityBeforeLockoutSchedule(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "inboxPath")]
pub struct InboxPath(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "initialized")]
pub struct Initialized(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "interactiveAgentIds")]
pub struct InteractiveAgentIds(Vec<String>);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "ipsThatCanSendEmailsThroughRest")]
pub struct IpsThatCanSendEmailsThroughRest(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "matchSecurityProtocol")]
pub struct MatchSecurityProtocol(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "maxFailedLogins")]
pub struct MaxFailedLogins(u64);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "maxFailedLoginsLockoutDuration")]
pub struct MaxFailedLoginsLockoutDuration(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "par")]
pub struct Par(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "passwordComplexity")]
pub struct PasswordComplexity(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "passwordComplexityMessage")]
pub struct PasswordComplexityMessage(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "passwordExpirationDate")]
pub struct PasswordExpirationDate(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "passwordExpirationInterval")]
pub struct PasswordExpirationInterval(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "passwordExpirationType")]
pub struct PasswordExpirationType(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "passwordHistoryDuration")]
pub struct PasswordHistoryDuration(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "passwordReuseRestriction")]
pub struct PasswordReuseRestriction(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "pathErrorWarning")]
pub struct PathErrorWarning(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "pipelinePath")]
pub struct PipelinePath(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "prearchivePath")]
pub struct PrearchivePath(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "primaryAdminUsername")]
pub struct PrimaryAdminUsername(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "processingUrl")]
pub struct ProcessingUrl(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "projectAllowAutoArchive")]
pub struct ProjectAllowAutoArchive(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "receivedFileUser")]
pub struct ReceivedFileUser(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "refreshGuestFrequency")]
pub struct RefreshGuestFrequency(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "reloadPrearcDatabaseOnStartup")]
pub struct ReloadPrearcDatabaseOnStartup(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "requireChangeJustification")]
pub struct RequireChangeJustification(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "requireEventName")]
pub struct RequireEventName(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "requireImageAssessorLabels")]
pub struct RequireImageAssessorLabels(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "requireLogin")]
pub struct RequireLogin(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "requireSaltedPasswords")]
pub struct RequireSaltedPasswords(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "resetFailedLoginsSchedule")]
pub struct ResetFailedLoginsSchedule(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "restrictUserListAccessToAdmins")]
pub struct RestrictUserListAccessToAdmins(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "roleRepositoryService")]
pub struct RoleRepositoryService(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "roleService")]
pub struct RoleService(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "scanTypeMapping")]
pub struct ScanTypeMapping(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "securityChannel")]
pub struct SecurityChannel(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "sessionTimeout")]
pub struct SessionTimeout(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "sessionTimeoutMessage")]
pub struct SessionTimeoutMessage(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "sessionXmlRebuilderInterval")]
pub struct SessionXmlRebuilderInterval(u64);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "sessionXmlRebuilderRepeat")]
pub struct SessionXmlRebuilderRepeat(u64);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "showChangeJustification")]
pub struct ShowChangeJustification(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteDescriptionPage")]
pub struct SiteDescriptionPage(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteDescriptionText")]
pub struct SiteDescriptionText(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteDescriptionType")]
pub struct SiteDescriptionType(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteHome")]
pub struct SiteHome(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteHomeLayout")]
pub struct SiteHomeLayout(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteId")]
pub struct SiteId(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteLandingLayout")]
pub struct SiteLandingLayout(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteLoginLanding")]
pub struct SiteLoginLanding(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteLogoPath")]
pub struct SiteLogoPath(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteUrl")]
pub struct SiteUrl(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteWideAlertMessage")]
pub struct SiteWideAlertMessage(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "siteWideAlertStatus")]
pub struct SiteWideAlertStatus(u64);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "siteWideAlertType")]
pub struct SiteWideAlertType(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "sitewideAnonymizationScript")]
pub struct SitewideAnonymizationScript(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "sitewidePetMr")]
pub struct SitewidePetMr(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "sitewidePetTracers")]
pub struct SitewidePetTracers(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "sitewideSeriesImportFilter")]
pub struct SitewideSeriesImportFilter(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "sitewideSeriesImportFilterMode")]
pub struct SitewideSeriesImportFilterMode(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "smtp_host")]
pub struct SmtpHost(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-LifeSpanReportsCacheUpdate-node")]
pub struct TaskLifeSpanReportsCacheUpdateNode(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-PcpStatusUpdate-node")]
pub struct TaskPcpStatusUpdateNode(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-PcpStatusUpdate-resolver")]
pub struct TaskPcpStatusUpdateResolver(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-PcpSubmitTask-node")]
pub struct TaskPcpSubmitTaskNode(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-SessionXMLRebuilder-node")]
pub struct TaskSessionXMLRebuilderNode(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-SessionXMLRebuilder-nodelist")]
pub struct TaskSessionXMLRebuilderNodelist(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-SessionXMLRebuilder-resolver")]
pub struct TaskSessionXMLRebuilderResolver(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-SessionXMLRebuilder-wait")]
pub struct TaskSessionXMLRebuilderWait(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-SpawnerInitializer-node")]
pub struct TaskSpawnerInitializerNode(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-SpawnerInitializer-resolver")]
pub struct TaskSpawnerInitializerResolver(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-XsyncScheduledSync-node")]
pub struct TaskXsyncScheduledSyncNode(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "task-XsyncScheduledSync-resolver")]
pub struct TaskXsyncScheduledSyncResolver(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiAllowAdvancedSearch")]
pub struct UiAllowAdvancedSearch(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiAllowBlockedSubjectAssessorView")]
pub struct UiAllowBlockedSubjectAssessorView(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiAllowNewUserComments")]
pub struct UiAllowNewUserComments(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiAllowNonAdminProjectCreation")]
pub struct UiAllowNonAdminProjectCreation(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiAllowProjectDelete")]
pub struct UiAllowProjectDelete(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiAllowQuarantine")]
pub struct UiAllowQuarantine(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiAllowScanAddition")]
pub struct UiAllowScanAddition(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiAllowScanTypeModification")]
pub struct UiAllowScanTypeModification(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiAllowSubjectCreateFromExptEdit")]
pub struct UiAllowSubjectCreateFromExptEdit(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiDebugExtensionPoints")]
pub struct UiDebugExtensionPoints(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiDisplaySeriesDescription")]
pub struct UiDisplaySeriesDescription(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "uiLoginFailureMessage")]
pub struct UiLoginFailureMessage(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiShowLeftBar")]
pub struct UiShowLeftBar(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "uiShowLeftBarAppletButton")]
pub struct UiShowLeftBarAppletButton(String);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiShowLeftBarBrowse")]
pub struct UiShowLeftBarBrowse(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiShowLeftBarFavorites")]
pub struct UiShowLeftBarFavorites(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiShowLeftBarProjects")]
pub struct UiShowLeftBarProjects(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiShowLeftBarSearch")]
pub struct UiShowLeftBarSearch(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiShowManageFiles")]
pub struct UiShowManageFiles(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "uiShowProjectManageFiles")]
pub struct UiShowProjectManageFiles(bool);

#[derive(Clone, Copy, Debug, Serialize, ModelField)]
#[serde(rename = "userRegistration")]
pub struct UserRegistration(bool);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "zipExtensions")]
pub struct ZipExtensions(String);

#[derive(Debug, Deserialize, Serialize)]
pub struct SiteConfig {
    #[serde(rename = "adminEmail")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_email: Option<AdminEmail>,
    #[serde(rename = "aliasTokenTimeout")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_token_timeout: Option<AliasTokenTimeout>,
    #[serde(rename = "aliasTokenTimeoutSchedule")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_token_timeout_schedule: Option<AliasTokenTimeoutSchedule>,
    #[serde(rename = "allowDataAdmins")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_data_admins: Option<AllowDataAdmins>,
    #[serde(rename = "allowHtmlResourceRendering")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_html_resource_rendering: Option<AllowHtmlResourceRendering>,
    #[serde(rename = "allowNonAdminsToClaimUnassignedSessions")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_non_admins_to_claim_unassigned_sessions: Option<AllowNonAdminsToClaimUnassignedSessions>,
    #[serde(rename = "archivePath")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_path: Option<ArchivePath>,
    #[serde(rename = "buildPath")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<BuildPath>,
    #[serde(rename = "cachePath")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_path: Option<CachePath>,
    #[serde(rename = "canResetFailedLoginsWithForgotPassword")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub can_reset_failed_logins_with_forgot_password: Option<CanResetFailedLoginsWithForgotPassword>,
    #[serde(rename = "checksums")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub checksums: Option<Checksums>,
    #[serde(rename = "concurrentMaxSessions")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_max_sessions: Option<ConcurrentMaxSessions>,
    #[serde(rename = "csrfEmailAlert")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub csrf_email_alert: Option<CsrfEmailAlert>,
    #[serde(rename = "dataPaths")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub data_paths: Option<DataPaths>,
    #[serde(rename = "emailVerification")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification: Option<EmailVerification>,
    #[serde(rename = "emailVerificationMessage")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_message: Option<EmailVerificationMessage>,
    #[serde(rename = "enableCsrfToken")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_csrf_token: Option<EnableCsrfToken>,
    #[serde(rename = "enableDicomReceiver")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dicom_receiver: Option<EnableDicomReceiver>,
    #[serde(rename = "enableDicomReceiverPropertyChangedListener")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dicom_receiver_property_changed_listener: Option<EnableDicomReceiverPropertyChangedListener>,
    #[serde(rename = "enableProjectAppletScript")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_project_applet_script: Option<EnableProjectAppletScript>,
    #[serde(rename = "enableSitewideAnonymizationScript")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sitewide_anonymization_script: Option<EnableSitewideAnonymizationScript>,
    #[serde(rename = "enableSitewideSeriesImportFilter")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sitewide_series_import_filter: Option<EnableSitewideSeriesImportFilter>,
    #[serde(rename = "failMergeOn")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_merge_on: Option<FailMergeOn>,
    #[serde(rename = "featureRepositoryService")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_repository_service: Option<FeatureRepositoryService>,
    #[serde(rename = "featureService")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_service: Option<FeatureService>,
    #[serde(rename = "ftpPath")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ftp_path: Option<FtpPath>,
    #[serde(rename = "imageSessionDisplayNamePlural")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub image_session_display_name_plural: Option<ImageSessionDisplayNamePlural>,
    #[serde(rename = "imageSessionDisplayNameSingular")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub image_session_display_name_singular: Option<ImageSessionDisplayNameSingular>,
    #[serde(rename = "inactivityBeforeLockout")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub inactivity_before_lockout: Option<InactivityBeforeLockout>,
    #[serde(rename = "inactivityBeforeLockoutSchedule")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub inactivity_before_lockout_schedule: Option<InactivityBeforeLockoutSchedule>,
    #[serde(rename = "inboxPath")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_path: Option<InboxPath>,
    #[serde(rename = "initialized")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub initialized: Option<Initialized>,
    #[serde(rename = "interactiveAgentIds")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive_agent_ids: Option<InteractiveAgentIds>,
    #[serde(rename = "ipsThatCanSendEmailsThroughRest")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ips_that_can_send_emails_through_rest: Option<IpsThatCanSendEmailsThroughRest>,
    #[serde(rename = "matchSecurityProtocol")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub match_security_protocol: Option<MatchSecurityProtocol>,
    #[serde(rename = "maxFailedLogins")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_logins: Option<MaxFailedLogins>,
    #[serde(rename = "maxFailedLoginsLockoutDuration")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_logins_lockout_duration: Option<MaxFailedLoginsLockoutDuration>,
    #[serde(rename = "par")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub par: Option<Par>,
    #[serde(rename = "passwordComplexity")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub password_complexity: Option<PasswordComplexity>,
    #[serde(rename = "passwordComplexityMessage")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub password_complexity_message: Option<PasswordComplexityMessage>,
    #[serde(rename = "passwordExpirationDate")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_date: Option<PasswordExpirationDate>,
    #[serde(rename = "passwordExpirationInterval")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_interval: Option<PasswordExpirationInterval>,
    #[serde(rename = "passwordExpirationType")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_type: Option<PasswordExpirationType>,
    #[serde(rename = "passwordHistoryDuration")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub password_history_duration: Option<PasswordHistoryDuration>,
    #[serde(rename = "passwordReuseRestriction")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reuse_restriction: Option<PasswordReuseRestriction>,
    #[serde(rename = "pathErrorWarning")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub path_error_warning: Option<PathErrorWarning>,
    #[serde(rename = "pipelinePath")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_path: Option<PipelinePath>,
    #[serde(rename = "prearchivePath")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub prearchive_path: Option<PrearchivePath>,
    #[serde(rename = "primaryAdminUsername")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_admin_username: Option<PrimaryAdminUsername>,
    #[serde(rename = "processingUrl")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_url: Option<ProcessingUrl>,
    #[serde(rename = "projectAllowAutoArchive")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub project_allow_auto_archive: Option<ProjectAllowAutoArchive>,
    #[serde(rename = "receivedFileUser")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub received_file_user: Option<ReceivedFileUser>,
    #[serde(rename = "refreshGuestFrequency")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_guest_frequency: Option<RefreshGuestFrequency>,
    #[serde(rename = "reloadPrearcDatabaseOnStartup")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub reload_prearc_database_on_startup: Option<ReloadPrearcDatabaseOnStartup>,
    #[serde(rename = "requireChangeJustification")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub require_change_justification: Option<RequireChangeJustification>,
    #[serde(rename = "requireEventName")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub require_event_name: Option<RequireEventName>,
    #[serde(rename = "requireImageAssessorLabels")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub require_image_assessor_labels: Option<RequireImageAssessorLabels>,
    #[serde(rename = "requireLogin")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub require_login: Option<RequireLogin>,
    #[serde(rename = "requireSaltedPasswords")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub require_salted_passwords: Option<RequireSaltedPasswords>,
    #[serde(rename = "resetFailedLoginsSchedule")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_failed_logins_schedule: Option<ResetFailedLoginsSchedule>,
    #[serde(rename = "restrictUserListAccessToAdmins")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_user_list_access_to_admins: Option<RestrictUserListAccessToAdmins>,
    #[serde(rename = "roleRepositoryService")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub role_repository_service: Option<RoleRepositoryService>,
    #[serde(rename = "roleService")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub role_service: Option<RoleService>,
    #[serde(rename = "scanTypeMapping")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type_mapping: Option<ScanTypeMapping>,
    #[serde(rename = "securityChannel")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub security_channel: Option<SecurityChannel>,
    #[serde(rename = "sessionTimeout")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<SessionTimeout>,
    #[serde(rename = "sessionTimeoutMessage")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout_message: Option<SessionTimeoutMessage>,
    #[serde(rename = "sessionXmlRebuilderInterval")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub session_xml_rebuilder_interval: Option<SessionXmlRebuilderInterval>,
    #[serde(rename = "sessionXmlRebuilderRepeat")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub session_xml_rebuilder_repeat: Option<SessionXmlRebuilderRepeat>,
    #[serde(rename = "showChangeJustification")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub show_change_justification: Option<ShowChangeJustification>,
    #[serde(rename = "siteDescriptionPage")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_description_page: Option<SiteDescriptionPage>,
    #[serde(rename = "siteDescriptionText")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_description_text: Option<SiteDescriptionText>,
    #[serde(rename = "siteDescriptionType")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_description_type: Option<SiteDescriptionType>,
    #[serde(rename = "siteHome")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_home: Option<SiteHome>,
    #[serde(rename = "siteHomeLayout")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_home_layout: Option<SiteHomeLayout>,
    #[serde(rename = "siteId")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<SiteId>,
    #[serde(rename = "siteLandingLayout")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_landing_layout: Option<SiteLandingLayout>,
    #[serde(rename = "siteLoginLanding")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_login_landing: Option<SiteLoginLanding>,
    #[serde(rename = "siteLogoPath")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_logo_path: Option<SiteLogoPath>,
    #[serde(rename = "siteUrl")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_url: Option<SiteUrl>,
    #[serde(rename = "siteWideAlertMessage")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_wide_alert_message: Option<SiteWideAlertMessage>,
    #[serde(rename = "siteWideAlertStatus")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_wide_alert_status: Option<SiteWideAlertStatus>,
    #[serde(rename = "siteWideAlertType")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub site_wide_alert_type: Option<SiteWideAlertType>,
    #[serde(rename = "sitewideAnonymizationScript")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub sitewide_anonymization_script: Option<SitewideAnonymizationScript>,
    #[serde(rename = "sitewidePetMr")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub sitewide_pet_mr: Option<SitewidePetMr>,
    #[serde(rename = "sitewidePetTracers")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub sitewide_pet_tracers: Option<SitewidePetTracers>,
    #[serde(rename = "sitewideSeriesImportFilter")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub sitewide_series_import_filter: Option<SitewideSeriesImportFilter>,
    #[serde(rename = "sitewideSeriesImportFilterMode")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub sitewide_series_import_filter_mode: Option<SitewideSeriesImportFilterMode>,
    #[serde(rename = "smtp_host")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_host: Option<SmtpHost>,
    #[serde(rename = "task-LifeSpanReportsCacheUpdate-node")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_life_span_reports_cache_updatenode: Option<TaskLifeSpanReportsCacheUpdateNode>,
    #[serde(rename = "task-PcpStatusUpdate-node")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_pcp_status_updatenode: Option<TaskPcpStatusUpdateNode>,
    #[serde(rename = "task-PcpStatusUpdate-resolver")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_pcp_status_updateresolver: Option<TaskPcpStatusUpdateResolver>,
    #[serde(rename = "task-PcpSubmitTask-node")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_pcp_submit_tasknode: Option<TaskPcpSubmitTaskNode>,
    #[serde(rename = "task-SessionXMLRebuilder-node")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_session_x_m_l_rebuildernode: Option<TaskSessionXMLRebuilderNode>,
    #[serde(rename = "task-SessionXMLRebuilder-nodelist")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_session_x_m_l_rebuildernodelist: Option<TaskSessionXMLRebuilderNodelist>,
    #[serde(rename = "task-SessionXMLRebuilder-resolver")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_session_x_m_l_rebuilderresolver: Option<TaskSessionXMLRebuilderResolver>,
    #[serde(rename = "task-SessionXMLRebuilder-wait")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_session_x_m_l_rebuilderwait: Option<TaskSessionXMLRebuilderWait>,
    #[serde(rename = "task-SpawnerInitializer-node")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_spawner_initializernode: Option<TaskSpawnerInitializerNode>,
    #[serde(rename = "task-SpawnerInitializer-resolver")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_spawner_initializerresolver: Option<TaskSpawnerInitializerResolver>,
    #[serde(rename = "task-XsyncScheduledSync-node")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_xsync_scheduled_syncnode: Option<TaskXsyncScheduledSyncNode>,
    #[serde(rename = "task-XsyncScheduledSync-resolver")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub task_xsync_scheduled_syncresolver: Option<TaskXsyncScheduledSyncResolver>,
    #[serde(rename = "uiAllowAdvancedSearch")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_allow_advanced_search: Option<UiAllowAdvancedSearch>,
    #[serde(rename = "uiAllowBlockedSubjectAssessorView")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_allow_blocked_subject_assessor_view: Option<UiAllowBlockedSubjectAssessorView>,
    #[serde(rename = "uiAllowNewUserComments")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_allow_new_user_comments: Option<UiAllowNewUserComments>,
    #[serde(rename = "uiAllowNonAdminProjectCreation")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_allow_non_admin_project_creation: Option<UiAllowNonAdminProjectCreation>,
    #[serde(rename = "uiAllowProjectDelete")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_allow_project_delete: Option<UiAllowProjectDelete>,
    #[serde(rename = "uiAllowQuarantine")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_allow_quarantine: Option<UiAllowQuarantine>,
    #[serde(rename = "uiAllowScanAddition")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_allow_scan_addition: Option<UiAllowScanAddition>,
    #[serde(rename = "uiAllowScanTypeModification")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_allow_scan_type_modification: Option<UiAllowScanTypeModification>,
    #[serde(rename = "uiAllowSubjectCreateFromExptEdit")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_allow_subject_create_from_expt_edit: Option<UiAllowSubjectCreateFromExptEdit>,
    #[serde(rename = "uiDebugExtensionPoints")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_debug_extension_points: Option<UiDebugExtensionPoints>,
    #[serde(rename = "uiDisplaySeriesDescription")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_display_series_description: Option<UiDisplaySeriesDescription>,
    #[serde(rename = "uiLoginFailureMessage")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_login_failure_message: Option<UiLoginFailureMessage>,
    #[serde(rename = "uiShowLeftBar")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_show_left_bar: Option<UiShowLeftBar>,
    #[serde(rename = "uiShowLeftBarAppletButton")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_show_left_bar_applet_button: Option<UiShowLeftBarAppletButton>,
    #[serde(rename = "uiShowLeftBarBrowse")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_show_left_bar_browse: Option<UiShowLeftBarBrowse>,
    #[serde(rename = "uiShowLeftBarFavorites")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_show_left_bar_favorites: Option<UiShowLeftBarFavorites>,
    #[serde(rename = "uiShowLeftBarProjects")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_show_left_bar_projects: Option<UiShowLeftBarProjects>,
    #[serde(rename = "uiShowLeftBarSearch")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_show_left_bar_search: Option<UiShowLeftBarSearch>,
    #[serde(rename = "uiShowManageFiles")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_show_manage_files: Option<UiShowManageFiles>,
    #[serde(rename = "uiShowProjectManageFiles")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_show_project_manage_files: Option<UiShowProjectManageFiles>,
    #[serde(rename = "userRegistration")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub user_registration: Option<UserRegistration>,
    #[serde(rename = "zipExtensions")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_extensions: Option<ZipExtensions>,
}

