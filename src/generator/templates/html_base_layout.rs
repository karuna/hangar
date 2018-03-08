pub static TEXT: &'static str = "{% import \"macros\" as macros %}
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\" />
  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
  <meta name=\"author\" content=\"Karuna Murti\">
  <meta name=\"description\" content=\"This is Hangar, an opionated full fledged web framework for rust.\">
  <title>{{header.title}}</title>
  <link rel=\"icon\" type=\"image/svg\" sizes=\"any\" href={{macros::asset_url(filename=\"favicon.svg\")}} />
  <link rel=\"stylesheet\" type=\"text/css\" href={{macros::asset_url(filename=\"application.css\")}} />
  <script src={{macros::asset_url(filename=\"application.js\")}}></script>
</head>
<body>
  <main class=\"wrapper\">
    <section class=\"container\" id=\"examples\">
      <div class=\"container\">
        {% block content %}{% endblock content %}
      </div>
    </section>
    <section class=\"container\" id=\"debug\">
      <pre>{{__tera_context}}</pre>
    </section>
  </main>
</body>
</html>
";