# rust-actix-tera

Tera is a **template engine** for **Rust** 
Based on [Jinja2](http://jinja.pocoo.org/) and the [Django template](https://docs.djangoproject.com/en/3.1/topics/templates/) language.

### Easy to use
Designed from the ground up to be easy to use and provide good error messages.  

### Performant
Template compilation and rendering are measured in nanoseconds.  

### Familiar
Used to Jinja2, Django templates, Liquid or Twig? You will feel right at home.  

## How it works

Use associated words to to read the template fields from a json file or a json object, like serde
```
<title>{% block title %}{% endblock title %}</title>
<ul>
{% for user in users -%}
  <li><a href="{{ user.url }}">{{ user.username }}</a></li>
{%- endfor %}
</ul>
```

## Installation

First check you have already Rust installed.  
Clone this repositorio [Git](https://github.com/ricardodarocha/rust-actix-tera.git)  
Run `cargo check cargo run`


```bash
cd rust-actix-tera>
cargo check cargo run
```

## Contributing

This is just an example of how to use tera. 

## License

[MIT](https://choosealicense.com/licenses/mit/)
