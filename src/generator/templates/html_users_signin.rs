pub static TEXT: &'static str = "{% extends \"base_layout\" %}

{% block content %}
<form action=\"/users/login\" method=\"post\">
  <label for=\"email\">Email</label>
  <input type=\"text\" name=\"email\" id=\"email\" /><br/>
  <label for=\"password\">Password</label>
  <input type=\"password\" name=\"password\" id=\"password\" /><br/>
  <input type=\"submit\" value=\"Submit\" />
</form>
{% endblock content %}";
