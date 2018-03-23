pub static TEXT: &'static str = "use std::collections::BTreeMap;
use std::env::temp_dir;
use std::fmt::Debug;
use std::fs::create_dir;
use std::io::Read;
use std::path::{Path, PathBuf};

use lettre::{EmailTransport, FileEmailTransport, SendableEmail, SendmailTransport, SmtpTransport};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::client::net::ClientTlsParameters;
use lettre::smtp::{ClientSecurity, ConnectionReuseParameters};
use lettre::smtp::extension::ClientId;
use lettre::stub::StubEmailTransport;
use lettre::smtp::client::net::DEFAULT_TLS_PROTOCOLS;

use native_tls::TlsConnector;

use rocket::config::{Table, Value};
use super::settings::{unwrap_int, unwrap_str};

pub enum Transport {
    File(FileEmailTransport),
    Sendmail(SendmailTransport),
    Smtp(SmtpTransport),
    Stub(StubEmailTransport),
}

pub struct Mailer(pub Transport);

pub struct MailerConfig(pub Table);

impl Mailer {
    pub fn new_from_config(mailer_config: &Table) -> Result<Mailer, String> {
        let enabled_key = \"enabled\";
        if !mailer_config.contains_key(enabled_key)
            || !mailer_config.get(enabled_key).unwrap().is_bool()
            || !mailer_config.get(enabled_key).unwrap().as_bool().unwrap()
        {
            return Err(String::from(\"Mailer is not enabled\"));
        }
        let transport_key = \"transport\";
        if !mailer_config.contains_key(transport_key)
            || !mailer_config.get(transport_key).unwrap().is_str()
        {
            generate_stub_email_transport()
        } else {
            match unwrap_str(mailer_config, transport_key) {
                \"file\" => generate_file_email_transport(mailer_config),
                \"sendmail\" => generate_sendmail_transport(mailer_config),
                \"smtp\" => generate_smtp_transport(mailer_config),
                _ => generate_stub_email_transport(),
            }
        }
    }

    pub fn send_email<'a, T: Read + 'a, U: SendableEmail<'a, T> + 'a>(
        &mut self,
        email: &'a U,
    ) -> Result<String, String> {
        match self.0 {
            Transport::File(ref mut file_transport) => stringify_result(file_transport.send(email)),
            Transport::Smtp(ref mut smtp_transport) => stringify_result(smtp_transport.send(email)),
            Transport::Sendmail(ref mut sendmail_transport) => {
                stringify_result(sendmail_transport.send(email))
            }
            Transport::Stub(ref mut stub_transport) => stringify_result(stub_transport.send(email)),
        }
    }
}

fn generate_tls_parameter(domain: &str) -> Result<ClientTlsParameters, String> {
    let mut tls_builder = match TlsConnector::builder() {
        Ok(builder) => builder,
        Err(e) => {
            return Err(format!(\"Failed to create TLS connector: {:?}\", e));
        }
    };

    if let Err(e) = tls_builder.supported_protocols(DEFAULT_TLS_PROTOCOLS) {
        return Err(format!(\"Failed to set supported protocols: {:?}\", e));
    }

    Ok(ClientTlsParameters::new(
        String::from(domain),
        tls_builder.build().unwrap(),
    ))
}

fn generate_client_security(
    mailer_config: &BTreeMap<String, Value>,
    domain: &str,
) -> Result<ClientSecurity, String> {
    let security_key = \"security\";
    if !mailer_config.contains_key(security_key) {
        Ok(ClientSecurity::None)
    } else {
        match generate_tls_parameter(domain) {
            Err(e) => Err(format!(\"Failed to generate tls_parameters: {:?}\", e)),
            Ok(tls_param) => match unwrap_str(mailer_config, security_key) {
                \"required\" => Ok(ClientSecurity::Required(tls_param)),
                \"wrapper\" => Ok(ClientSecurity::Wrapper(tls_param)),
                _ => Ok(ClientSecurity::Opportunistic(tls_param)),
            },
        }
    }
}

fn generate_reuse(
    mailer_config: &BTreeMap<String, Value>,
) -> Result<ConnectionReuseParameters, String> {
    let reuse_key = \"reuse\";
    if !mailer_config.contains_key(reuse_key) {
        Ok(ConnectionReuseParameters::NoReuse)
    } else {
        if !mailer_config.get(reuse_key).unwrap().is_str() {
            return Err(String::from(\"Not a valid configuration.\"));
        }

        match unwrap_str(mailer_config, reuse_key) {
            \"reuseunlimited\" => Ok(ConnectionReuseParameters::ReuseUnlimited),
            \"reuselimited\" => {
                let reuse_limit_key = \"reuse_limit\";
                if !mailer_config.contains_key(reuse_limit_key)
                    || !mailer_config.get(reuse_limit_key).unwrap().is_integer()
                {
                    return Err(String::from(\"Not a valid configuration.\"));
                }
                let reuse_limit = unwrap_int(mailer_config, reuse_limit_key) as u16;
                Ok(ConnectionReuseParameters::ReuseLimited(reuse_limit))
            }
            _ => Ok(ConnectionReuseParameters::NoReuse),
        }
    }
}

fn generate_auth_mechanism(mailer_config: &BTreeMap<String, Value>) -> Result<Mechanism, String> {
    let mechanism_key = \"mechanism\";
    if !mailer_config.contains_key(mechanism_key) {
        Ok(Mechanism::Plain)
    } else {
        if !mailer_config.get(mechanism_key).unwrap().is_str() {
            return Err(String::from(\"Not a valid configuration.\"));
        }

        match unwrap_str(mailer_config, mechanism_key) {
            \"login\" => Ok(Mechanism::Login),
            \"crammd5\" => Ok(Mechanism::CramMd5),
            _ => Ok(Mechanism::Plain),
        }
    }
}

fn generate_username_password(
    mailer_config: &BTreeMap<String, Value>,
) -> Result<Credentials, String> {
    let username_key = \"username\";
    let password_key = \"password\";
    if !mailer_config.contains_key(username_key) || !mailer_config.contains_key(password_key)
        || !mailer_config.get(username_key).unwrap().is_str()
        || !mailer_config.get(password_key).unwrap().is_str()
    {
        Err(String::from(\"Not a valid configuration.\"))
    } else {
        Ok(Credentials::new(
            String::from(unwrap_str(mailer_config, username_key)),
            String::from(unwrap_str(mailer_config, password_key)),
        ))
    }
}

fn generate_domain(mailer_config: &BTreeMap<String, Value>) -> Result<&str, String> {
    let domain_key = \"domain\";

    if !mailer_config.contains_key(domain_key) {
        return Ok(\"localhost\");
    }
    if !mailer_config.get(domain_key).unwrap().is_str() {
        return Err(String::from(\"Not a valid configuration.\"));
    }
    Ok(unwrap_str(mailer_config, domain_key))
}

fn generate_sendmail_path(
    mailer_config: &BTreeMap<String, Value>,
    sendmail_path_key: &str,
) -> Result<String, String> {
    if !mailer_config.get(sendmail_path_key).unwrap().is_str() {
        return Err(String::from(\"Not a valid configuration.\"));
    }
    let path_str = unwrap_str(mailer_config, sendmail_path_key);
    // Basic checking since SendmailTransport constructor doesn't raise any error
    // TODO: check permission on unix
    let path_file = Path::new(path_str);
    if !path_file.exists() || !path_file.is_file() {
        return Err(String::from(\"sendmail path does not exist.\"));
    }
    Ok(path_str.to_string())
}

fn generate_mail_send_dir(
    mailer_config: &BTreeMap<String, Value>,
    mail_send_dir_key: &str,
) -> Result<PathBuf, String> {
    if !mailer_config.contains_key(mail_send_dir_key) {
        Ok(temp_dir())
    } else {
        if !mailer_config.get(mail_send_dir_key).unwrap().is_str() {
            return Err(String::from(\"Not a valid configuration.\"));
        }
        let path_str = unwrap_str(mailer_config, mail_send_dir_key);
        let path_dir = Path::new(path_str);
        if !path_dir.exists() && create_dir(path_str).is_err() {
            return Err(String::from(\"Cannot create directory for email.\"));
        }
        Ok(path_dir.to_path_buf())
    }
}

fn generate_file_email_transport(
    mailer_config: &BTreeMap<String, Value>,
) -> Result<Mailer, String> {
    let mail_send_dir_key = \"mail_send_dir\";
    generate_mail_send_dir(mailer_config, mail_send_dir_key)
        .map(|mail_send_dir| Mailer(Transport::File(FileEmailTransport::new(mail_send_dir))))
}

fn generate_sendmail_transport(mailer_config: &BTreeMap<String, Value>) -> Result<Mailer, String> {
    let sendmail_path_key = \"sendmail_path\";
    if !mailer_config.contains_key(sendmail_path_key) {
        Ok(Mailer(Transport::Sendmail(SendmailTransport::new())))
    } else {
        generate_sendmail_path(mailer_config, sendmail_path_key).map(|path_str| {
            Mailer(Transport::Sendmail(SendmailTransport::new_with_command(
                path_str,
            )))
        })
    }
}

fn generate_smtp_transport(mailer_config: &BTreeMap<String, Value>) -> Result<Mailer, String> {
    let domain = match generate_domain(mailer_config) {
        Ok(domain) => domain,
        Err(e) => return Err(e),
    };

    let client_security = match generate_client_security(mailer_config, domain) {
        Ok(client_security) => client_security,
        Err(e) => return Err(e),
    };

    let reuse = match generate_reuse(mailer_config) {
        Ok(reuse) => reuse,
        Err(e) => return Err(e),
    };

    let auth_mechanism = match generate_auth_mechanism(mailer_config) {
        Ok(auth_mechanism) => auth_mechanism,
        Err(e) => return Err(e),
    };

    let credential = match generate_username_password(mailer_config) {
        Ok(credential) => credential,
        Err(e) => return Err(e),
    };

    // There's a possibility to raise panic here
    let smtp_transport = SmtpTransport::builder(domain, client_security)
        .unwrap()
        .hello_name(ClientId::Domain(domain.to_string()))
        .credentials(credential)
        .authentication_mechanism(auth_mechanism)
        .connection_reuse(reuse)
        .build();

    Ok(Mailer(Transport::Smtp(smtp_transport)))
}

fn generate_stub_email_transport() -> Result<Mailer, String> {
    Ok(Mailer(Transport::Stub(StubEmailTransport::new_positive())))
}

fn stringify_result<O: Debug, E: Debug>(result: Result<O, E>) -> Result<String, String> {
    match result {
        Ok(ok) => Ok(format!(\"{:?}\", ok).to_string()),
        Err(err) => Err(format!(\"{:?}\", err).to_string()),
    }
}
";
