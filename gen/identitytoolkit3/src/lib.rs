// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Identity Toolkit* crate version *0.1.1+20141009*, where *20141009* is the exact revision of the *identitytoolkit:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.1*.
//! 
//! Everything else about the *Identity Toolkit* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/identity-toolkit/v3/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/identitytoolkit3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.IdentityToolkit.html) ... 
//! 
//! * [relyingparty](struct.Relyingparty.html)
//!  * [*create auth uri*](struct.RelyingpartyCreateAuthUriCall.html), [*delete account*](struct.RelyingpartyDeleteAccountCall.html), [*download account*](struct.RelyingpartyDownloadAccountCall.html), [*get account info*](struct.RelyingpartyGetAccountInfoCall.html), [*get oob confirmation code*](struct.RelyingpartyGetOobConfirmationCodeCall.html), [*get public keys*](struct.RelyingpartyGetPublicKeyCall.html), [*reset password*](struct.RelyingpartyResetPasswordCall.html), [*set account info*](struct.RelyingpartySetAccountInfoCall.html), [*upload account*](struct.RelyingpartyUploadAccountCall.html), [*verify assertion*](struct.RelyingpartyVerifyAssertionCall.html) and [*verify password*](struct.RelyingpartyVerifyPasswordCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.IdentityToolkit.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.relyingparty().verify_assertion(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-identitytoolkit3 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-identitytoolkit3" as identitytoolkit3;
//! use identitytoolkit3::IdentitytoolkitRelyingpartyVerifyAssertionRequest;
//! use identitytoolkit3::Result;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use identitytoolkit3::IdentityToolkit;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req: IdentitytoolkitRelyingpartyVerifyAssertionRequest = Default::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.relyingparty().verify_assertion(&req)
//!              .doit();
//! 
//! match result {
//!     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!     Result::MissingToken => println!("OAuth2: Missing Token"),
//!     Result::Cancelled => println!("Operation cancelled by user"),
//!     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     Result::Success(_) => println!("Success (value doesn't print)"),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downlods
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via json. Optionals are used to indicate that partial requests are responses are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifyable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(core,io,thread_sleep)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate "yup-oauth2" as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, ResourceMethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all IdentityToolkit related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartyVerifyAssertionRequest;
/// use identitytoolkit3::Result;
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use identitytoolkit3::IdentityToolkit;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartyVerifyAssertionRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().verify_assertion(&req)
///              .doit();
/// 
/// match result {
///     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
///     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///     Result::MissingToken => println!("OAuth2: Missing Token"),
///     Result::Cancelled => println!("Operation cancelled by user"),
///     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     Result::Success(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct IdentityToolkit<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for IdentityToolkit<C, NC, A> {}

impl<'a, C, NC, A> IdentityToolkit<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> IdentityToolkit<C, NC, A> {
        IdentityToolkit {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.1".to_string(),
            _m: PhantomData
        }
    }

    pub fn relyingparty(&'a self) -> RelyingpartyMethods<'a, C, NC, A> {
        RelyingpartyMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.1`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// Request of getting a code for user confirmation (reset password, change email etc.)
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get oob confirmation code relyingparty](struct.RelyingpartyGetOobConfirmationCodeCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct Relyingparty {
    /// The fixed string "identitytoolkit#relyingparty".    
    pub kind: Option<String>,
    /// The request type.    
    #[serde(alias="requestType")]
    pub request_type: Option<String>,
    /// The user's Gitkit login token for email change.    
    #[serde(alias="idToken")]
    pub id_token: Option<String>,
    /// The recaptcha challenge presented to the user.    
    pub challenge: Option<String>,
    /// The new email if the code is for email change.    
    #[serde(alias="newEmail")]
    pub new_email: Option<String>,
    /// The IP address of the user.    
    #[serde(alias="userIp")]
    pub user_ip: Option<String>,
    /// The email of the user.    
    pub email: Option<String>,
    /// The recaptcha response from the user.    
    #[serde(alias="captchaResp")]
    pub captcha_resp: Option<String>,
}

impl RequestValue for Relyingparty {}


/// Response of creating the IDP authentication URL.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create auth uri relyingparty](struct.RelyingpartyCreateAuthUriCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct CreateAuthUriResponse {
    /// The fixed string identitytoolkit#CreateAuthUriResponse".    
    pub kind: String,
    /// True if captcha is required.    
    #[serde(alias="captchaRequired")]
    pub captcha_required: bool,
    /// The URI used by the IDP to authenticate the user.    
    #[serde(alias="authUri")]
    pub auth_uri: String,
    /// Whether the user is registered if the identifier is an email.    
    pub registered: bool,
    /// The provider ID of the auth URI.    
    #[serde(alias="providerId")]
    pub provider_id: String,
    /// True if the authUri is for user's existing provider.    
    #[serde(alias="forExistingProvider")]
    pub for_existing_provider: bool,
}

impl ResponseResult for CreateAuthUriResponse {}


/// Request to set the account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set account info relyingparty](struct.RelyingpartySetAccountInfoCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct IdentitytoolkitRelyingpartySetAccountInfoRequest {
    /// The name of the user.    
    #[serde(alias="displayName")]
    pub display_name: Option<String>,
    /// The local ID of the user.    
    #[serde(alias="localId")]
    pub local_id: Option<String>,
    /// Mark the user to upgrade to federated login.    
    #[serde(alias="upgradeToFederatedLogin")]
    pub upgrade_to_federated_login: Option<bool>,
    /// Response to the captcha.    
    #[serde(alias="captchaResponse")]
    pub captcha_response: Option<String>,
    /// The GITKit token of the authenticated user.    
    #[serde(alias="idToken")]
    pub id_token: Option<String>,
    /// Mark the email as verified or not.    
    #[serde(alias="emailVerified")]
    pub email_verified: Option<bool>,
    /// The captcha challenge.    
    #[serde(alias="captchaChallenge")]
    pub captcha_challenge: Option<String>,
    /// The associated IDPs of the user.    
    pub provider: Option<Vec<String>>,
    /// The out-of-band code of the change email request.    
    #[serde(alias="oobCode")]
    pub oob_code: Option<String>,
    /// The new password of the user.    
    pub password: Option<String>,
    /// The email of the user.    
    pub email: Option<String>,
}

impl RequestValue for IdentitytoolkitRelyingpartySetAccountInfoRequest {}


/// Request to delete account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete account relyingparty](struct.RelyingpartyDeleteAccountCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct IdentitytoolkitRelyingpartyDeleteAccountRequest {
    /// The local ID of the user.    
    #[serde(alias="localId")]
    pub local_id: Option<String>,
}

impl RequestValue for IdentitytoolkitRelyingpartyDeleteAccountRequest {}


/// Response of getting a code for user confirmation (reset password, change email etc.).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get oob confirmation code relyingparty](struct.RelyingpartyGetOobConfirmationCodeCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct GetOobConfirmationCodeResponse {
    /// The fixed string "identitytoolkit#GetOobConfirmationCodeResponse".    
    pub kind: String,
    /// The code to be send to the user.    
    #[serde(alias="oobCode")]
    pub oob_code: String,
}

impl ResponseResult for GetOobConfirmationCodeResponse {}


/// Request to get the account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get account info relyingparty](struct.RelyingpartyGetAccountInfoCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct IdentitytoolkitRelyingpartyGetAccountInfoRequest {
    /// The GITKit token of the authenticated user.    
    #[serde(alias="idToken")]
    pub id_token: Option<String>,
    /// The list of emails of the users to inquiry.    
    pub email: Option<Vec<String>>,
    /// The list of local ID's of the users to inquiry.    
    #[serde(alias="localId")]
    pub local_id: Option<Vec<String>>,
}

impl RequestValue for IdentitytoolkitRelyingpartyGetAccountInfoRequest {}


/// Respone of downloading accounts in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download account relyingparty](struct.RelyingpartyDownloadAccountCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DownloadAccountResponse {
    /// The next page token. To be used in a subsequent request to return the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The fixed string "identitytoolkit#DownloadAccountResponse".    
    pub kind: String,
    /// The user accounts data.    
    pub users: Vec<UserInfo>,
}

impl ResponseResult for DownloadAccountResponse {}


/// Request to reset the password.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset password relyingparty](struct.RelyingpartyResetPasswordCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct IdentitytoolkitRelyingpartyResetPasswordRequest {
    /// The new password inputted by the user.    
    #[serde(alias="newPassword")]
    pub new_password: Option<String>,
    /// The old password inputted by the user.    
    #[serde(alias="oldPassword")]
    pub old_password: Option<String>,
    /// The confirmation code.    
    #[serde(alias="oobCode")]
    pub oob_code: Option<String>,
    /// The email address of the user.    
    pub email: Option<String>,
}

impl RequestValue for IdentitytoolkitRelyingpartyResetPasswordRequest {}


/// Response of verifying the IDP assertion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify assertion relyingparty](struct.RelyingpartyVerifyAssertionCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VerifyAssertionResponse {
    /// The unique ID identifies the IdP account.    
    #[serde(alias="federatedId")]
    pub federated_id: String,
    /// The RP local ID if it's already been mapped to the IdP account identified by the federated ID.    
    #[serde(alias="localId")]
    pub local_id: String,
    /// The URI of the public accessible profiel picture.    
    #[serde(alias="photoUrl")]
    pub photo_url: String,
    /// It's the identifier param in the createAuthUri request if the identifier is an email. It can be used to check whether the user input email is different from the asserted email.    
    #[serde(alias="inputEmail")]
    pub input_email: String,
    /// The value is true if the IDP is also the email provider. It means the user owns the email.    
    #[serde(alias="emailVerified")]
    pub email_verified: bool,
    /// Whether the assertion is from a non-trusted IDP and need account linking confirmation.    
    #[serde(alias="needConfirmation")]
    pub need_confirmation: bool,
    /// The custom scheme used by mobile app.    
    #[serde(alias="appScheme")]
    pub app_scheme: String,
    /// The full name of the user.    
    #[serde(alias="fullName")]
    pub full_name: String,
    /// URL for OTA app installation.    
    #[serde(alias="appInstallationUrl")]
    pub app_installation_url: String,
    /// The fixed string "identitytoolkit#VerifyAssertionResponse".    
    pub kind: String,
    /// The display name of the user.    
    #[serde(alias="displayName")]
    pub display_name: String,
    /// The first name of the user.    
    #[serde(alias="firstName")]
    pub first_name: String,
    /// The language preference of the user.    
    pub language: String,
    /// The ID token.    
    #[serde(alias="idToken")]
    pub id_token: String,
    /// When action is 'map', contains the idps which can be used for confirmation.    
    #[serde(alias="verifiedProvider")]
    pub verified_provider: Vec<String>,
    /// The scope for the OpenID OAuth extension.    
    #[serde(alias="oauthScope")]
    pub oauth_scope: String,
    /// The user approved request token for the OpenID OAuth extension.    
    #[serde(alias="oauthRequestToken")]
    pub oauth_request_token: String,
    /// The birth date of the IdP account.    
    #[serde(alias="dateOfBirth")]
    pub date_of_birth: String,
    /// The original email stored in the mapping storage. It's returned when the federated ID is associated to a different email.    
    #[serde(alias="originalEmail")]
    pub original_email: String,
    /// The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. If the "providerId" param is set to OpenID OP identifer other than the whilte listed IdPs the OP identifier is returned. If the "identifier" param is federated ID in the createAuthUri request. The domain part of the federated ID is returned.    
    #[serde(alias="providerId")]
    pub provider_id: String,
    /// The opaque value used by the client to maintain context info between the authentication request and the IDP callback.    
    pub context: String,
    /// The last name of the user.    
    #[serde(alias="lastName")]
    pub last_name: String,
    /// The action code.    
    pub action: String,
    /// The timezone of the user.    
    #[serde(alias="timeZone")]
    pub time_zone: String,
    /// The nick name of the user.    
    #[serde(alias="nickName")]
    pub nick_name: String,
    /// The email returned by the IdP. NOTE: The federated login user may not own the email.    
    pub email: String,
    /// It's true if the email is recycled.    
    #[serde(alias="emailRecycled")]
    pub email_recycled: bool,
}

impl ResponseResult for VerifyAssertionResponse {}


/// The error encountered while processing the account info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct UploadAccountResponseError {
    /// The index of the malformed account, starting from 0.    
    pub index: i32,
    /// Detailed error message for the account info.    
    pub message: String,
}

impl NestedType for UploadAccountResponseError {}
impl Part for UploadAccountResponseError {}


/// Respone of uploading accounts in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload account relyingparty](struct.RelyingpartyUploadAccountCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct UploadAccountResponse {
    /// The fixed string "identitytoolkit#UploadAccountResponse".    
    pub kind: String,
    /// The error encountered while processing the account info.    
    pub error: Vec<UploadAccountResponseError>,
}

impl ResponseResult for UploadAccountResponse {}


/// Request to upload user account in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload account relyingparty](struct.RelyingpartyUploadAccountCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct IdentitytoolkitRelyingpartyUploadAccountRequest {
    /// The password hash algorithm.    
    #[serde(alias="hashAlgorithm")]
    pub hash_algorithm: Option<String>,
    /// Memory cost for hash calculation. Used by scrypt similar algorithms.    
    #[serde(alias="memoryCost")]
    pub memory_cost: Option<i32>,
    /// The account info to be stored.    
    pub users: Option<Vec<UserInfo>>,
    /// The salt separator.    
    #[serde(alias="saltSeparator")]
    pub salt_separator: Option<String>,
    /// The key for to hash the password.    
    #[serde(alias="signerKey")]
    pub signer_key: Option<String>,
    /// Rounds for hash calculation. Used by scrypt and similar algorithms.    
    pub rounds: Option<i32>,
}

impl RequestValue for IdentitytoolkitRelyingpartyUploadAccountRequest {}


/// Request of verifying the password.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify password relyingparty](struct.RelyingpartyVerifyPasswordCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VerifyPasswordResponse {
    /// The fixed string "identitytoolkit#VerifyPasswordResponse".    
    pub kind: String,
    /// The name of the user.    
    #[serde(alias="displayName")]
    pub display_name: String,
    /// The RP local ID if it's already been mapped to the IdP account identified by the federated ID.    
    #[serde(alias="localId")]
    pub local_id: String,
    /// The URI of the user's photo at IdP    
    #[serde(alias="photoUrl")]
    pub photo_url: String,
    /// The GITKit token for authenticated user.    
    #[serde(alias="idToken")]
    pub id_token: String,
    /// Whether the email is registered.    
    pub registered: bool,
    /// The email returned by the IdP. NOTE: The federated login user may not own the email.    
    pub email: String,
}

impl ResponseResult for VerifyPasswordResponse {}


/// Response of resetting the password.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset password relyingparty](struct.RelyingpartyResetPasswordCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ResetPasswordResponse {
    /// The fixed string "identitytoolkit#ResetPasswordResponse".    
    pub kind: String,
    /// The user's email.    
    pub email: String,
}

impl ResponseResult for ResetPasswordResponse {}


/// Respone of deleting account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete account relyingparty](struct.RelyingpartyDeleteAccountCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct DeleteAccountResponse {
    /// The fixed string "identitytoolkit#DeleteAccountResponse".    
    pub kind: String,
}

impl ResponseResult for DeleteAccountResponse {}


/// Request to get the IDP authentication URL.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create auth uri relyingparty](struct.RelyingpartyCreateAuthUriCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct IdentitytoolkitRelyingpartyCreateAuthUriRequest {
    /// Optional realm for OpenID protocol. The sub string "scheme://domain:port" of the param "continueUri" is used if this is not set.    
    #[serde(alias="openidRealm")]
    pub openid_realm: Option<String>,
    /// The relying party OAuth client ID.    
    #[serde(alias="clientId")]
    pub client_id: Option<String>,
    /// The native app package for OTA installation.    
    #[serde(alias="otaApp")]
    pub ota_app: Option<String>,
    /// The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier.    
    #[serde(alias="providerId")]
    pub provider_id: Option<String>,
    /// The opaque value used by the client to maintain context info between the authentication request and the IDP callback.    
    pub context: Option<String>,
    /// The app ID of the mobile app, base64(CERT_SHA1):PACKAGE_NAME for Android, BUNDLE_ID for iOS.    
    #[serde(alias="appId")]
    pub app_id: Option<String>,
    /// The URI to which the IDP redirects the user after the federated login flow.    
    #[serde(alias="continueUri")]
    pub continue_uri: Option<String>,
    /// The email or federated ID of the user.    
    pub identifier: Option<String>,
}

impl RequestValue for IdentitytoolkitRelyingpartyCreateAuthUriRequest {}


/// The IDP of the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserInfoProviderUserInfo {
    /// User's identifier at IDP.    
    #[serde(alias="federatedId")]
    pub federated_id: String,
    /// The IdP ID. For white listed IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier.    
    #[serde(alias="providerId")]
    pub provider_id: String,
    /// The user's display name at the IDP.    
    #[serde(alias="displayName")]
    pub display_name: String,
    /// The user's photo url at the IDP.    
    #[serde(alias="photoUrl")]
    pub photo_url: String,
}

impl NestedType for UserInfoProviderUserInfo {}
impl Part for UserInfoProviderUserInfo {}


/// Respone of getting public keys.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get public keys relyingparty](struct.RelyingpartyGetPublicKeyCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct IdentitytoolkitRelyingpartyGetPublicKeysResponse(HashMap<String, String>);

impl ResponseResult for IdentitytoolkitRelyingpartyGetPublicKeysResponse {}


/// Request to verify the password.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify password relyingparty](struct.RelyingpartyVerifyPasswordCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct IdentitytoolkitRelyingpartyVerifyPasswordRequest {
    /// Response to the captcha.    
    #[serde(alias="captchaResponse")]
    pub captcha_response: Option<String>,
    /// The captcha challenge.    
    #[serde(alias="captchaChallenge")]
    pub captcha_challenge: Option<String>,
    /// The password inputed by the user.    
    pub password: Option<String>,
    /// The email of the user.    
    pub email: Option<String>,
    /// The GITKit token for the non-trusted IDP, which is to be confirmed by the user.    
    #[serde(alias="pendingIdToken")]
    pub pending_id_token: Option<String>,
}

impl RequestValue for IdentitytoolkitRelyingpartyVerifyPasswordRequest {}


/// Request to download user account in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download account relyingparty](struct.RelyingpartyDownloadAccountCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct IdentitytoolkitRelyingpartyDownloadAccountRequest {
    /// The token for the next page. This should be taken from the previous response.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The max number of results to return in the response.    
    #[serde(alias="maxResults")]
    pub max_results: Option<u32>,
}

impl RequestValue for IdentitytoolkitRelyingpartyDownloadAccountRequest {}


/// The user's profiles at the associated IdPs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SetAccountInfoResponseProviderUserInfo {
    /// The IdP ID. For whitelisted IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier.    
    #[serde(alias="providerId")]
    pub provider_id: String,
    /// The user's display name at the IDP.    
    #[serde(alias="displayName")]
    pub display_name: String,
    /// The user's photo url at the IDP.    
    #[serde(alias="photoUrl")]
    pub photo_url: String,
}

impl NestedType for SetAccountInfoResponseProviderUserInfo {}
impl Part for SetAccountInfoResponseProviderUserInfo {}


/// Response of getting account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get account info relyingparty](struct.RelyingpartyGetAccountInfoCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct GetAccountInfoResponse {
    /// The fixed string "identitytoolkit#GetAccountInfoResponse".    
    pub kind: String,
    /// The info of the users.    
    pub users: Vec<UserInfo>,
}

impl ResponseResult for GetAccountInfoResponse {}


/// Request to verify the IDP assertion.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify assertion relyingparty](struct.RelyingpartyVerifyAssertionCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct IdentitytoolkitRelyingpartyVerifyAssertionRequest {
    /// The URI to which the IDP redirects the user back. It may contain federated login result params added by the IDP.    
    #[serde(alias="requestUri")]
    pub request_uri: Option<String>,
    /// The post body if the request is a HTTP POST.    
    #[serde(alias="postBody")]
    pub post_body: Option<String>,
    /// The GITKit token for the non-trusted IDP pending to be confirmed by the user.    
    #[serde(alias="pendingIdToken")]
    pub pending_id_token: Option<String>,
}

impl RequestValue for IdentitytoolkitRelyingpartyVerifyAssertionRequest {}


/// Respone of setting the account information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set account info relyingparty](struct.RelyingpartySetAccountInfoCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SetAccountInfoResponse {
    /// The Gitkit id token to login the newly sign up user.    
    #[serde(alias="idToken")]
    pub id_token: String,
    /// The fixed string "identitytoolkit#SetAccountInfoResponse".    
    pub kind: String,
    /// The name of the user.    
    #[serde(alias="displayName")]
    pub display_name: String,
    /// The email of the user.    
    pub email: String,
    /// The user's profiles at the associated IdPs.    
    #[serde(alias="providerUserInfo")]
    pub provider_user_info: Vec<SetAccountInfoResponseProviderUserInfo>,
}

impl ResponseResult for SetAccountInfoResponse {}


/// Template for an individual account info.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// The name of the user.    
    #[serde(alias="displayName")]
    pub display_name: String,
    /// The local ID of the user.    
    #[serde(alias="localId")]
    pub local_id: String,
    /// The URL of the user profile photo.    
    #[serde(alias="photoUrl")]
    pub photo_url: String,
    /// Whether the email has been verified.    
    #[serde(alias="emailVerified")]
    pub email_verified: bool,
    /// The user's hashed password.    
    #[serde(alias="passwordHash")]
    pub password_hash: String,
    /// Version of the user's password.    
    pub version: i32,
    /// The IDP of the user.    
    #[serde(alias="providerUserInfo")]
    pub provider_user_info: Vec<UserInfoProviderUserInfo>,
    /// The timestamp when the password was last updated.    
    #[serde(alias="passwordUpdatedAt")]
    pub password_updated_at: f64,
    /// The user's password salt.    
    pub salt: String,
    /// The email of the user.    
    pub email: String,
}

impl Part for UserInfo {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *relyingparty* resources.
/// It is not used directly, but through the `IdentityToolkit` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-identitytoolkit3" as identitytoolkit3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use identitytoolkit3::IdentityToolkit;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create_auth_uri(...)`, `delete_account(...)`, `download_account(...)`, `get_account_info(...)`, `get_oob_confirmation_code(...)`, `get_public_keys(...)`, `reset_password(...)`, `set_account_info(...)`, `upload_account(...)`, `verify_assertion(...)` and `verify_password(...)`
/// // to build up your call.
/// let rb = hub.relyingparty();
/// # }
/// ```
pub struct RelyingpartyMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for RelyingpartyMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a code for user action confirmation.    
    pub fn get_oob_confirmation_code(&self, request: &Relyingparty) -> RelyingpartyGetOobConfirmationCodeCall<'a, C, NC, A> {
        RelyingpartyGetOobConfirmationCodeCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get token signing public key.    
    pub fn get_public_keys(&self) -> RelyingpartyGetPublicKeyCall<'a, C, NC, A> {
        RelyingpartyGetPublicKeyCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates the URI used by the IdP to authenticate the user.    
    pub fn create_auth_uri(&self, request: &IdentitytoolkitRelyingpartyCreateAuthUriRequest) -> RelyingpartyCreateAuthUriCall<'a, C, NC, A> {
        RelyingpartyCreateAuthUriCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Verifies the assertion returned by the IdP.    
    pub fn verify_assertion(&self, request: &IdentitytoolkitRelyingpartyVerifyAssertionRequest) -> RelyingpartyVerifyAssertionCall<'a, C, NC, A> {
        RelyingpartyVerifyAssertionCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batch upload existing user accounts.    
    pub fn upload_account(&self, request: &IdentitytoolkitRelyingpartyUploadAccountRequest) -> RelyingpartyUploadAccountCall<'a, C, NC, A> {
        RelyingpartyUploadAccountCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the account info.    
    pub fn get_account_info(&self, request: &IdentitytoolkitRelyingpartyGetAccountInfoRequest) -> RelyingpartyGetAccountInfoCall<'a, C, NC, A> {
        RelyingpartyGetAccountInfoCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reset password for a user.    
    pub fn reset_password(&self, request: &IdentitytoolkitRelyingpartyResetPasswordRequest) -> RelyingpartyResetPasswordCall<'a, C, NC, A> {
        RelyingpartyResetPasswordCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batch download user accounts.    
    pub fn download_account(&self, request: &IdentitytoolkitRelyingpartyDownloadAccountRequest) -> RelyingpartyDownloadAccountCall<'a, C, NC, A> {
        RelyingpartyDownloadAccountCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Set account info for a user.    
    pub fn set_account_info(&self, request: &IdentitytoolkitRelyingpartySetAccountInfoRequest) -> RelyingpartySetAccountInfoCall<'a, C, NC, A> {
        RelyingpartySetAccountInfoCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete user account.    
    pub fn delete_account(&self, request: &IdentitytoolkitRelyingpartyDeleteAccountRequest) -> RelyingpartyDeleteAccountCall<'a, C, NC, A> {
        RelyingpartyDeleteAccountCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Verifies the user entered password.    
    pub fn verify_password(&self, request: &IdentitytoolkitRelyingpartyVerifyPasswordRequest) -> RelyingpartyVerifyPasswordCall<'a, C, NC, A> {
        RelyingpartyVerifyPasswordCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Get a code for user action confirmation.
///
/// A builder for the *getOobConfirmationCode* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::Relyingparty;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Relyingparty = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().get_oob_confirmation_code(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartyGetOobConfirmationCodeCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: Relyingparty,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyGetOobConfirmationCodeCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyGetOobConfirmationCodeCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetOobConfirmationCodeResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.getOobConfirmationCode", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/getOobConfirmationCode".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &Relyingparty) -> RelyingpartyGetOobConfirmationCodeCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyGetOobConfirmationCodeCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyGetOobConfirmationCodeCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Get token signing public key.
///
/// A builder for the *getPublicKeys* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().get_public_keys()
///              .doit();
/// # }
/// ```
pub struct RelyingpartyGetPublicKeyCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyGetPublicKeyCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyGetPublicKeyCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, IdentitytoolkitRelyingpartyGetPublicKeysResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.getPublicKeys", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/publicKeys".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyGetPublicKeyCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyGetPublicKeyCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Creates the URI used by the IdP to authenticate the user.
///
/// A builder for the *createAuthUri* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartyCreateAuthUriRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartyCreateAuthUriRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().create_auth_uri(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartyCreateAuthUriCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: IdentitytoolkitRelyingpartyCreateAuthUriRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyCreateAuthUriCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyCreateAuthUriCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CreateAuthUriResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.createAuthUri", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/createAuthUri".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &IdentitytoolkitRelyingpartyCreateAuthUriRequest) -> RelyingpartyCreateAuthUriCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyCreateAuthUriCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyCreateAuthUriCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Verifies the assertion returned by the IdP.
///
/// A builder for the *verifyAssertion* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartyVerifyAssertionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartyVerifyAssertionRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().verify_assertion(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartyVerifyAssertionCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: IdentitytoolkitRelyingpartyVerifyAssertionRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyVerifyAssertionCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyVerifyAssertionCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, VerifyAssertionResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.verifyAssertion", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyAssertion".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &IdentitytoolkitRelyingpartyVerifyAssertionRequest) -> RelyingpartyVerifyAssertionCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyVerifyAssertionCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyVerifyAssertionCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Batch upload existing user accounts.
///
/// A builder for the *uploadAccount* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartyUploadAccountRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartyUploadAccountRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().upload_account(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartyUploadAccountCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: IdentitytoolkitRelyingpartyUploadAccountRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyUploadAccountCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyUploadAccountCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UploadAccountResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.uploadAccount", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/uploadAccount".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &IdentitytoolkitRelyingpartyUploadAccountRequest) -> RelyingpartyUploadAccountCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyUploadAccountCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyUploadAccountCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Returns the account info.
///
/// A builder for the *getAccountInfo* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartyGetAccountInfoRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartyGetAccountInfoRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().get_account_info(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartyGetAccountInfoCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: IdentitytoolkitRelyingpartyGetAccountInfoRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyGetAccountInfoCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyGetAccountInfoCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GetAccountInfoResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.getAccountInfo", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/getAccountInfo".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &IdentitytoolkitRelyingpartyGetAccountInfoRequest) -> RelyingpartyGetAccountInfoCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyGetAccountInfoCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyGetAccountInfoCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Reset password for a user.
///
/// A builder for the *resetPassword* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartyResetPasswordRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartyResetPasswordRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().reset_password(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartyResetPasswordCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: IdentitytoolkitRelyingpartyResetPasswordRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyResetPasswordCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyResetPasswordCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ResetPasswordResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.resetPassword", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/resetPassword".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &IdentitytoolkitRelyingpartyResetPasswordRequest) -> RelyingpartyResetPasswordCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyResetPasswordCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyResetPasswordCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Batch download user accounts.
///
/// A builder for the *downloadAccount* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartyDownloadAccountRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartyDownloadAccountRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().download_account(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartyDownloadAccountCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: IdentitytoolkitRelyingpartyDownloadAccountRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyDownloadAccountCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyDownloadAccountCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DownloadAccountResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.downloadAccount", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/downloadAccount".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &IdentitytoolkitRelyingpartyDownloadAccountRequest) -> RelyingpartyDownloadAccountCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyDownloadAccountCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyDownloadAccountCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Set account info for a user.
///
/// A builder for the *setAccountInfo* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartySetAccountInfoRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartySetAccountInfoRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().set_account_info(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartySetAccountInfoCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: IdentitytoolkitRelyingpartySetAccountInfoRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartySetAccountInfoCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartySetAccountInfoCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SetAccountInfoResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.setAccountInfo", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/setAccountInfo".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &IdentitytoolkitRelyingpartySetAccountInfoRequest) -> RelyingpartySetAccountInfoCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartySetAccountInfoCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartySetAccountInfoCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Delete user account.
///
/// A builder for the *deleteAccount* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartyDeleteAccountRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartyDeleteAccountRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().delete_account(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartyDeleteAccountCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: IdentitytoolkitRelyingpartyDeleteAccountRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyDeleteAccountCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyDeleteAccountCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DeleteAccountResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.deleteAccount", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/deleteAccount".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &IdentitytoolkitRelyingpartyDeleteAccountRequest) -> RelyingpartyDeleteAccountCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyDeleteAccountCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyDeleteAccountCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}


/// Verifies the user entered password.
///
/// A builder for the *verifyPassword* method supported by a *relyingparty* resource.
/// It is not used directly, but through a `RelyingpartyMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-identitytoolkit3" as identitytoolkit3;
/// use identitytoolkit3::IdentitytoolkitRelyingpartyVerifyPasswordRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use identitytoolkit3::IdentityToolkit;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IdentityToolkit::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: IdentitytoolkitRelyingpartyVerifyPasswordRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.relyingparty().verify_password(&req)
///              .doit();
/// # }
/// ```
pub struct RelyingpartyVerifyPasswordCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a IdentityToolkit<C, NC, A>,
    _request: IdentitytoolkitRelyingpartyVerifyPasswordRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, NC, A> CallBuilder for RelyingpartyVerifyPasswordCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RelyingpartyVerifyPasswordCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, VerifyPasswordResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "identitytoolkit.relyingparty.verifyPassword", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyPassword".to_string();
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Result::MissingAPIKey
            }
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &IdentitytoolkitRelyingpartyVerifyPasswordRequest) -> RelyingpartyVerifyPasswordCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RelyingpartyVerifyPasswordCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RelyingpartyVerifyPasswordCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

}

