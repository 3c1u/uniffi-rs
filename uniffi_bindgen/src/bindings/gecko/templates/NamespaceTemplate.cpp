// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

{% import "macros.cpp" as cpp %}

#include "mozilla/dom/{{ ci.namespace()|class_name_webidl }}.h"

namespace mozilla {
namespace dom {

{% for func in functions %}
/* static */
{% call cpp::decl_return_type(func) %} {{ ci.namespace()|class_name_cpp }}::{{ func.name()|fn_name_cpp }}(
  {% call cpp::decl_static_method_args(func) %}
) {
  {%- if func.throws().is_some() %}
  RustError err = {0, nullptr};
  {% endif %}
  {% match func.return_type() -%}{%- when Some with (type_) -%}const {{ type_|ret_type_ffi }} loweredRetVal_ = {%- else -%}{% endmatch %}{{ func.ffi_func().name() }}(
    {%- let args = func.arguments() %}
    {% call cpp::to_ffi_args(args) -%}
    {%- if func.throws().is_some() %}
    {% if !args.is_empty() %},{% endif %}&err
    {% endif %}
  );
  {%- if func.throws().is_some() %}
  if (err.mCode) {
    aRv.ThrowOperationError(err.mMessage);
    {% call cpp::bail(func) %}
  }
  {%- endif %}
  {% call cpp::return(func, "loweredRetVal_") %}
}
{% endfor %}

}  // namespace dom
}  // namespace mozilla
