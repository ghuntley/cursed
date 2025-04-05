    /// Parse type parameters for generic functions/structs ([T], [A, B], etc.)
    fn parse_type_parameters(&mut self) -> Result<Vec<ast::Identifier>, Error> {
        let mut type_parameters = Vec::new();
        
        // The current token should be '['
        if self.current_token != Token::LBracket {
            return Err(Error::from_str(
                &format!("Expected '[' to start type parameters, got {:?}", self.current_token)
            ));
        }
        
        // Move past '['
        self.next_token()?;
        
        // Empty type parameters case
        if self.current_token == Token::RBracket {
            self.next_token()?; // Consume ']'
            return Ok(type_parameters);
        }
        
        // Parse first type parameter
        if let Token::Identifier(param_name) = &self.current_token {
            type_parameters.push(ast::Identifier {
                token: self.current_token.token_literal(),
                value: param_name.clone(),
            });
            self.next_token()?;
        } else {
            return Err(Error::from_str(
                &format!("Expected identifier as type parameter, got {:?}", self.current_token)
            ));
        }
        
        // Parse remaining type parameters
        while self.current_token == Token::Comma {
            self.next_token()?; // Consume ','
            
            if let Token::Identifier(param_name) = &self.current_token {
                type_parameters.push(ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: param_name.clone(),
                });
                self.next_token()?;
            } else {
                return Err(Error::from_str(
                    &format!("Expected identifier after comma in type parameters, got {:?}", self.current_token)
                ));
            }
        }
        
        // Expect closing bracket
        if self.current_token != Token::RBracket {
            return Err(Error::from_str(
                &format!("Expected ']' after type parameters, got {:?}", self.current_token)
            ));
        }
        
        // Move past ']'
        self.next_token()?;
        
        Ok(type_parameters)
    }