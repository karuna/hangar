pub static TEXT: &'static str = "{% extends \"base_layout\" %}

{% block content %}
  <h1>User</h1>
  <div>{{ body.email }}</div>
  <div>{{ body.id }}</div>
{% endblock content %}
";
