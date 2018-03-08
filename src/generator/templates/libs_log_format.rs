pub static TEXT: &'static str = "use chrono::prelude::Utc;

use rocket::Request;
use rocket::Response;

pub static CLF_TIME: &'static str = \"%d/%b/%Y:%H:%M:%S %z\";
// ISO-8601, e.g. javascript's new Date().toISOString()
pub static ISO_8601_UTC: &'static str = \"%Y-%m-%dT%H:%M:%S%.3fZ\";
// When offset from UTC != 0, then the offset is displayed instead of \"Z\".
pub static ISO_8601_OFFSET: &'static str = \"%Y-%m-%dT%H:%M:%S%.3f%:z\";

// TODO: fix :proto, :size, :userid, :user
pub static COMMON_LOG_FORMAT: &'static str = \":host :userid :user [:time] \\\":method :url :proto\\\" :status :size\";

pub struct Common;

impl Common {
    pub fn log(req: &Request, res: &Response)-> String {
        let host = req.remote();
        let host = if host.is_some() {
            format!(\"{}\", host.unwrap())
        } else {
            format!(\"\")
        };
        let method = req.method().as_str();
        let url = req.uri().as_str();
        let status = format!(\"{}\", res.status().code);
        let proto = \"-\";
        let size = \"0\";
        let userid = \"-\";
        let user = \"-\";
        let time = Utc::now().format(CLF_TIME);
        COMMON_LOG_FORMAT
            .replace(\":host\", &host)
            .replace(\":userid\", userid)
            .replace(\":user\", user)
            .replace(\":time\", &format!(\"{}\", time))
            .replace(\":method\", method)
            .replace(\":url\", url)
            .replace(\":proto\", proto)
            .replace(\":status\", &status)
            .replace(\":size\", size)
    }
}";
