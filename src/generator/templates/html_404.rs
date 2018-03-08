pub static TEXT: &'static str = "{% extends \"base_layout\" %}

{% block content %}
  <h1>404</h1>
  <p><strong>File not found</strong></p>

  <p>
    The page you are requestiong is not available.
  </p>

  <a href=\"/home\">Home</a>
{% endblock content %}
";
