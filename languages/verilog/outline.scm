; Modules
(module_declaration
    (module_ansi_header
        (module_keyword) @context
        name: (_) @name)) @item

; Always blocks
(always_construct
  (always_keyword) @context
  (statement
    (statement_item
      (seq_block (simple_identifier) @name)?
      (procedural_timing_control_statement
        (statement_or_null
          (statement
            (statement_item (seq_block (simple_identifier) @name)))))?)
    )) @item

; for(genvar ...)
(loop_generate_construct
  "for" @context
  (genvar_initialization (simple_identifier) @context.extra)
  (generate_block name: (_) @name)?) @item

; Module instances
(module_instantiation
   instance_type: (_) @context
   (hierarchical_instance
     (name_of_instance
     instance_name: (_) @name))) @item

; Typedefs
(type_declaration
  "typedef" @context
  type_name: (_) @name) @item

; Struct and union members
(struct_union_member
  (data_type_or_void (data_type)? @context)
  (list_of_variable_decl_assignments) @name
  ) @item

; Initial blocks
(initial_construct
  "initial" @context
  (statement_or_null
    (statement
      (statement_item
        (seq_block
          (simple_identifier) @name)?)))) @item

; Classes
(class_declaration
  "class" @context
  name: (_) @name
) @item

; Class properties
(class_property
  (data_declaration
    (data_type_or_implicit) @context
    (list_of_variable_decl_assignments) @name)) @item

; Functions and tasks,
; either as class methods
; or in any other place
(
  "extern"? @context
  (method_qualifier)? @context
  [
   (function_declaration
     "function" @context
     (lifetime)? @context
     (function_body_declaration
       (data_type_or_void) @context
       (class_scope)? @name
       name: (_) @name)
     )
   (function_prototype
     "function" @context
     (data_type_or_void) @context
     name: (_) @name)
   (task_prototype
     "task" @context
     name: (_) @name)
   (task_declaration
     "task" @context
     (task_body_declaration
       (class_scope)? @name
       name: (_) @name))
   ]) @item
