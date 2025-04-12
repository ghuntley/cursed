impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get the current variable scope, if any
    pub fn current_scope(&self) -> Option<&VariableScope<'ctx>> {
        self.var_scopes.last()
    }

    /// Get a mutable reference to the current variable scope, if any
    pub fn current_scope_mut(&mut self) -> Option<&mut VariableScope<'ctx>> {
        self.var_scopes.last_mut()
    }

    /// Push a new variable scope
    pub fn push_scope(&mut self, scope: VariableScope<'ctx>) {
        self.var_scopes.push(scope);
    }

    /// Pop the current variable scope
    pub fn pop_scope(&mut self) -> Option<VariableScope<'ctx>> {
        self.var_scopes.pop()
    }
}