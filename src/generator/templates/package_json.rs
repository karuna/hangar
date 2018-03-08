pub static TEXT: &'static str = "{
  \"name\": \"rocket-hangar\",
  \"version\": \"1.0.0\",
  \"description\": \"Rust web framework, using [Rocket](https://rocket.rs), [Diesel](https://diesel.rs), and [Yew](https://github.com/DenisKolodin/yew)\",
  \"main\": \"index.js\",
  \"scripts\": {
    \"start-js\": \"parcel -d src/assets client/assets/application.js\",
    \"start-css\": \"parcel -d src/assets client/assets/application.css\"
  },
  \"repository\": {
    \"type\": \"git\",
    \"url\": \"git+ssh://git@gitlab.com/karuna/hangar.git\"
  },
  \"keywords\": [],
  \"author\": \"{{authors_name_email}}\",
  \"license\": \"ISC\",
  \"bugs\": {
    \"url\": \"https://gitlab.com/karuna/hangar/issues\"
  },
  \"homepage\": \"https://gitlab.com/karuna/hangar#README\",
  \"dependencies\": {
    \"parcel-bundler\": \"^1.5.1\"
  },
  \"devDependencies\": {
    \"parcel-plugin-cargo-web\": \"^0.1.0\"
  }
}
";
