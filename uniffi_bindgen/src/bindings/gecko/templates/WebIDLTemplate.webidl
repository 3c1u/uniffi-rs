// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

{%- for rec in ci.iter_record_definitions() %}
dictionary {{ rec.name()|class_name_webidl }} {
  {%- for field in rec.fields() %}
  required {{ field.type_()|type_webidl }} {{ field.name()|var_name_webidl }};
  {%- endfor %}
};
{% endfor %}

{%- for e in ci.iter_enum_definitions() %}
enum {{ e.name()|class_name_webidl }} {
  {% for variant in e.variants() %}
  "{{ variant|enum_variant_webidl }}"{%- if !loop.last %}, {% endif %}
  {% endfor %}
};
{% endfor %}

{%- let functions = ci.iter_function_definitions() %}
{%- if !functions.is_empty() %}
[ChromeOnly, Exposed=Window]
namespace {{ ci.namespace()|class_name_webidl }} {
  {#-
  // We'll need to figure out how to handle async methods. One option is
  // to declare them as `async foo()`, or an `[Async]` or `[BackgroundThread]`
  // attribute in the UniFFI IDL. Kotlin, Swift, and Python can ignore that
  // anno; Gecko will generate a method that returns a `Promise` instead, and
  // dispatches the task to the background thread.
  #}
  {% for func in functions %}
  {%- if func.throws().is_some() %}
  [Throws]
  {% endif %}
  {%- match func.return_type() -%}{%- when Some with (type_) %}{{ type_|type_webidl }}{% when None %}void{% endmatch %} {{ func.name()|fn_name_webidl }}(
    {%- for arg in func.arguments() %}
    {{ arg.type_()|type_webidl }} {{ arg.name() }}{%- if !loop.last %}, {% endif %}
    {%- endfor %}
  );
  {% endfor %}
};
{% endif -%}

{%- for obj in ci.iter_object_definitions() %}
[ChromeOnly, Exposed=Window]
interface {{ obj.name()|class_name_webidl }} {
  {%- for cons in obj.constructors() %}
  {%- if cons.throws().is_some() %}
  [Throws]
  {% endif %}
  constructor(
      {%- for arg in cons.arguments() %}
      {{ arg.type_()|type_webidl }} {{ arg.name() }}{%- if !loop.last %}, {% endif %}
      {%- endfor %}
  );
  {%- endfor %}

  {% for meth in obj.methods() -%}
  [Throws]
  {%- match meth.return_type() -%}{%- when Some with (type_) %}{{ type_|type_webidl }}{% when None %}void{% endmatch %} {{ meth.name()|fn_name_webidl }}(
      {%- for arg in meth.arguments() %}
      {{ arg.type_()|type_webidl }} {{ arg.name() }}{%- if !loop.last %}, {% endif %}
      {%- endfor %}
  );
  {% endfor %}
};
{% endfor %}
