#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, Symbol, symbol_short};

// Employee structure to store employee details
#[contracttype]
#[derive(Clone)]
pub struct Employee {
    pub emp_address: Address,
    pub salary: i128,           // Salary in stroops (1 XLM = 10,000,000 stroops)
    pub is_active: bool,
    pub last_paid: u64,         // Timestamp of last payment
}

// Mapping employee address to employee data
#[contracttype]
pub enum EmployeeBook {
    Employee(Address)
}

// Symbol for tracking total employees
const TOTAL_EMPLOYEES: Symbol = symbol_short!("TOT_EMP");
// Symbol for contract owner
const OWNER: Symbol = symbol_short!("OWNER");

#[contract]
pub struct PayrollContract;

#[contractimpl]
impl PayrollContract {
    
    // Initialize contract with owner
    pub fn initialize(env: Env, owner: Address) {
        // Ensure contract is not already initialized
        if env.storage().instance().has(&OWNER) {
            log!(&env, "Contract already initialized");
            panic!("Contract already initialized");
        }
        
        owner.require_auth();
        
        // Set the contract owner
        env.storage().instance().set(&OWNER, &owner);
        env.storage().instance().set(&TOTAL_EMPLOYEES, &0u64);
        
        env.storage().instance().extend_ttl(5000, 5000);
        log!(&env, "Payroll Contract Initialized");
    }
    
    // Add or update an employee (only owner can do this)
    pub fn add_employee(env: Env, emp_address: Address, salary: i128) {
        let owner: Address = env.storage().instance().get(&OWNER)
            .expect("Contract not initialized");
        
        owner.require_auth();
        
        let key = EmployeeBook::Employee(emp_address.clone());
        let time = env.ledger().timestamp();
        
        // Check if employee already exists
        let employee_exists = env.storage().instance().has(&key);
        
        let employee = Employee {
            emp_address: emp_address.clone(),
            salary,
            is_active: true,
            last_paid: time,
        };
        
        env.storage().instance().set(&key, &employee);
        
        if !employee_exists {
            let mut total: u64 = env.storage().instance().get(&TOTAL_EMPLOYEES).unwrap_or(0);
            total += 1;
            env.storage().instance().set(&TOTAL_EMPLOYEES, &total);
        }
        
        env.storage().instance().extend_ttl(5000, 5000);
        log!(&env, "Employee added/updated successfully");
    }
    
    // Process payroll for an employee (only owner can trigger)
    pub fn process_payroll(env: Env, emp_address: Address) {
        let owner: Address = env.storage().instance().get(&OWNER)
            .expect("Contract not initialized");
        
        owner.require_auth();
        
        let key = EmployeeBook::Employee(emp_address.clone());
        let mut employee: Employee = env.storage().instance().get(&key)
            .expect("Employee not found");
        
        if !employee.is_active {
            log!(&env, "Employee is not active");
            panic!("Employee is not active");
        }
        
        let time = env.ledger().timestamp();
        
        // Update last paid timestamp
        employee.last_paid = time;
        env.storage().instance().set(&key, &employee);
        
        env.storage().instance().extend_ttl(5000, 5000);
        log!(&env, "Payroll processed for employee. Amount: {}", employee.salary);
    }
    
    // Deactivate an employee (only owner can do this)
    pub fn deactivate_employee(env: Env, emp_address: Address) {
        let owner: Address = env.storage().instance().get(&OWNER)
            .expect("Contract not initialized");
        
        owner.require_auth();
        
        let key = EmployeeBook::Employee(emp_address.clone());
        let mut employee: Employee = env.storage().instance().get(&key)
            .expect("Employee not found");
        
        employee.is_active = false;
        env.storage().instance().set(&key, &employee);
        
        env.storage().instance().extend_ttl(5000, 5000);
        log!(&env, "Employee deactivated");
    }
    
    // View employee details
    pub fn view_employee(env: Env, emp_address: Address) -> Employee {
        let key = EmployeeBook::Employee(emp_address.clone());
        
        env.storage().instance().get(&key).unwrap_or(Employee {
            emp_address: emp_address.clone(),
            salary: 0,
            is_active: false,
            last_paid: 0,
        })
    }
    
    // Get total number of employees
    pub fn get_total_employees(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_EMPLOYEES).unwrap_or(0)
    }
}
