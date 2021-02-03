use super::block_builder::BlockBuilder;
use super::typed_expression::*;
use crate::ir::*;
use crate::types::{self, Type};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug, Default)]
pub struct ModuleBuilder {
    name_index: Rc<AtomicU64>,
    variable_declarations: Rc<RefCell<Vec<VariableDeclaration>>>,
    function_declarations: Rc<RefCell<Vec<FunctionDeclaration>>>,
    variable_definitions: Rc<RefCell<Vec<VariableDefinition>>>,
    function_definitions: Rc<RefCell<Vec<FunctionDefinition>>>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            name_index: AtomicU64::new(0).into(),
            variable_declarations: RefCell::new(vec![]).into(),
            function_declarations: RefCell::new(vec![]).into(),
            variable_definitions: RefCell::new(vec![]).into(),
            function_definitions: RefCell::new(vec![]).into(),
        }
    }

    pub fn as_module(&self) -> Module {
        Module::new(
            self.variable_declarations.as_ref().borrow().clone(),
            self.function_declarations.as_ref().borrow().clone(),
            self.variable_definitions.as_ref().borrow().clone(),
            self.function_definitions.as_ref().borrow().clone(),
        )
    }

    pub fn declare_variable(
        &self,
        name: impl Into<String>,
        type_: impl Into<Type>,
    ) -> TypedExpression {
        let name = name.into();
        let type_ = type_.into();

        self.variable_declarations
            .borrow_mut()
            .push(VariableDeclaration::new(&name, type_.clone()));

        TypedExpression::new(Variable::new(name), types::Pointer::new(type_))
    }

    pub fn declare_function(
        &self,
        name: impl Into<String>,
        type_: types::Function,
    ) -> TypedExpression {
        let name = name.into();

        self.function_declarations
            .borrow_mut()
            .push(FunctionDeclaration::new(&name, type_.clone()));

        TypedExpression::new(Variable::new(name), type_)
    }

    pub fn define_variable(
        &self,
        name: impl Into<String>,
        body: impl Into<TypedExpression>,
        mutable: bool,
        global: bool,
    ) -> TypedExpression {
        let name = name.into();
        let body = body.into();

        self.variable_definitions
            .borrow_mut()
            .push(VariableDefinition::new(
                &name,
                body.expression().clone(),
                body.type_().clone(),
                mutable,
                global,
            ));

        TypedExpression::new(
            Variable::new(name),
            types::Pointer::new(body.type_().clone()),
        )
    }

    pub fn define_function(
        &self,
        name: impl Into<String>,
        arguments: Vec<Argument>,
        body: impl Fn(BlockBuilder) -> Block,
        result_type: impl Into<Type>,
        global: bool,
    ) -> TypedExpression {
        let result_type = result_type.into();
        let name = name.into();
        let body = body(BlockBuilder::new(self.clone()));

        self.function_definitions
            .borrow_mut()
            .push(FunctionDefinition::new(
                &name,
                arguments.clone(),
                body,
                result_type.clone(),
                global,
            ));

        TypedExpression::new(
            Variable::new(name),
            types::Function::new(
                arguments
                    .iter()
                    .map(|argument| argument.type_().clone())
                    .collect(),
                result_type,
            ),
        )
    }

    pub fn define_anonymous_function(
        &self,
        arguments: Vec<Argument>,
        body: impl Fn(BlockBuilder) -> Block,
        result_type: impl Into<Type>,
    ) -> TypedExpression {
        self.define_function(self.generate_name(), arguments, body, result_type, false)
    }

    pub fn generate_name(&self) -> String {
        format!("_metir_{:x}", self.name_index.fetch_add(1, Ordering::SeqCst))
    }
}
