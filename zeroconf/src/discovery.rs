use std::any::Any;
use std::sync::Arc;

/// Callback invoked from [`MdnsBrowser`] once a service has been discovered and resolved.
///
/// # Arguments
/// * `discovered_service` - The service that was disovered
/// * `context` - The optional user context passed through using [`MdnsBrowser::set_context()`]
///
/// [`MdnsBrowser`]: struct.MdnsBrowser.html
/// [`MdnsBrowser::set_context()`]: struct.MdnsBrowser.html#method.set_context
pub type ServiceDiscoveredCallback = dyn Fn(ServiceDiscovery, Option<Arc<dyn Any>>);

/// Represents a service that has been discovered by a [`MdnsBrowser`].
///
/// [`MdnsBrowser`]: struct.MdnsBrowser.html
#[derive(Debug, Getters, Builder, BuilderDelegate, Serialize, Deserialize)]
pub struct ServiceDiscovery {
    name: String,
    kind: String,
    domain: String,
    host_name: String,
    address: String,
    port: u16,
}
