#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol, String, symbol_short};

// Define Symbols for storage keys
const LOAN_LIST: Symbol = symbol_short!("LOAN_LIST");
const BALANCES: Symbol = symbol_short!("BALANCES");

#[contract]
pub struct MicroloanContract;

#[contractimpl]
impl MicroloanContract {
    /// This function allows users to issue a new loan.
    /// The loan terms include the amount, interest rate, and repayment duration.
    pub fn issue_loan(env: Env, loan_id: u64, borrower_id: u64, amount: u64, interest_rate: u64, duration: u64) {
        let time = env.ledger().timestamp();
        let loan_info = (borrower_id, amount, interest_rate, duration, time, false); // loan not yet repaid
        env.storage().instance().set(&loan_id, &loan_info);

        // Optionally update balances
        let mut balances: u64 = env.storage().instance().get(&BALANCES).unwrap_or(0);
        balances -= amount; // Deduct loan amount from the platform's balance
        env.storage().instance().set(&BALANCES, &balances);

        log!(&env, "Loan Issued: ID = {}, Borrower ID = {}, Amount = {}, Interest Rate = {}, Duration = {}", 
             loan_id, borrower_id, amount, interest_rate, duration);
    }

    /// This function allows users to repay a loan.
    /// The repayment includes the amount paid.
    pub fn repay_loan(env: Env, loan_id: u64, repayment_amount: u64) {
        let mut loan_info = Self::view_loan(env.clone(), loan_id);
        if loan_info.5 { // Check if the loan is already repaid
            log!(&env, "Loan ID: {} has already been repaid", loan_id);
            panic!("Loan already repaid");
        }
        
        let (borrower_id, amount, interest_rate, duration, issue_time, repaid) = loan_info;
        let total_due = amount + (amount * interest_rate / 100); // Calculate total due with interest
        
        if repayment_amount >= total_due {
            // Mark loan as repaid
            let updated_loan_info = (borrower_id, amount, interest_rate, duration, issue_time, true);
            env.storage().instance().set(&loan_id, &updated_loan_info);
            
            // Update balances
            let mut balances: u64 = env.storage().instance().get(&BALANCES).unwrap_or(0);
            balances += repayment_amount; // Add repayment to the platform's balance
            env.storage().instance().set(&BALANCES, &balances);
            
            log!(&env, "Loan ID: {} repaid successfully", loan_id);
        } else {
            log!(&env, "Insufficient repayment amount for Loan ID: {}", loan_id);
            panic!("Insufficient repayment amount");
        }
    }

    /// This function retrieves the information of a specific loan.
    pub fn view_loan(env: Env, loan_id: u64) -> (u64, u64, u64, u64, u64, bool) {
        env.storage().instance().get(&loan_id).unwrap_or((0, 0, 0, 0, 0, false))
    }
}
