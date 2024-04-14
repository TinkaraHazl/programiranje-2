use super::models::{AExpr, BinaryOperation};
impl AExpr {
    pub fn evaluate(&self) -> i64 {{
        match self {
            AExpr::Num(k) => *k,
            AExpr::Variable(neznanka) => panic!("Neznank ne sme biti.")
            AExpr::BinOp(levo, operacija, desno) => match operacija {
                BinaryOperation::Add => levo.evaluate() + desno.evaluate(),
                BinaryOperation::Sub => levo.evaluate() - desno.evaluate(),
                BinaryOperation::Mul => levo.evaluate() * desno.evaluate(),
                BinaryOperation::Pow => i64::pow(levo.evaluate(), desno.evaluate())
            }
        }
    };

    pub fn evaluate_map(
        &self,
        vars: &std::collections::HashMap<String, Option<i64>>,
    ) -> Option<i64> {
        panic!("Implement variable evaluation")
    }
}
}