pub static TEXT: &'static str = "{% extends \"base_layout\" %}

{% block content %}
<form action=\"/users/register\" method=\"post\">
  <label for=\"email\">Email</label>
  <input type=\"text\" name=\"email\" id=\"email\" value=\"{{body.email}}\" /><br/>
  <label for=\"password\">Password</label>
  <input type=\"password\" name=\"password\" id=\"password\" /><br/>
  <label for=\"password_confirmation\">Password Confirmation</label>
  <input type=\"password\" name=\"password_confirmation\" id=\"password_confirmation\"><br/>
  <input type=\"submit\" value=\"Submit\" />
</form>
{% endblock content %}";
