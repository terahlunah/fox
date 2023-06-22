#[cfg(test)]
mod tests {
    use fox_core::engine::engine::Engine;
    use fox_core::engine::env::Env;
    use fox_core::engine::error::Error;
    use fox_core::engine::expr::Expr;
    use fox_core::engine::function::Function;
    use fox_core::engine::stack::Stack;
    use fox_core::engine::value::Value;
    use fox_core::stdlib::list::List;

    #[test]
    fn tests() {
        fn list_new(
            defs: &Env<Function>,
            locals: &Env<Value>,
            stack: &mut Stack,
        ) -> Result<(), Error> {
            let obj = List(vec![]);
            stack.push_native(obj);
            Ok(())
        }
        fn list_push(
            defs: &Env<Function>,
            locals: &Env<Value>,
            stack: &mut Stack,
        ) -> Result<(), Error> {
            println!("{:?}", stack);
            let v = stack.pop()?;
            println!("{:?}", stack);
            let mut obj = stack.pop()?;
            obj.as_native_typed_mut::<List>()?.0.push(v);
            stack.push(obj);
            Ok(())
        }

        let mut engine = Engine::new();
        engine.definitions.set(
            "List.new".to_string(),
            Function::native("List.new", list_new),
        );
        engine.definitions.set(
            "List.push".to_string(),
            Function::native("List.push", list_push),
        );

        engine
            .eval(&[
                Expr::symbol("List.new"),
                Expr::num(42.0),
                Expr::symbol("List.push"),
            ])
            .unwrap();

        println!("Stack: {:?}", engine.stack)
    }
}
