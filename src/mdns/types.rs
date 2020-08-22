trait TMdnsService {
    fn start(&self);
}

trait TMdnsBrowser {
    fn start(&self);
}

trait TServiceResolution {
    fn name(&self) -> &str;
    
    fn kind(&self) -> &str;

    fn domain(&self) -> &str;

    fn host_name(&self) -> &str;

    fn address(&self) -> &str;

    fn port(&self) -> u16;

    fn is_local(&self) -> bool;
}
