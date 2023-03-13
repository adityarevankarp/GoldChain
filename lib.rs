
solana_program::declare_id!("your_program_id_here");


#[derive(Clone, Debug, PartialEq)]
struct Order {
    id: u64,
    customer_pubkey: Pubkey,
    retailer_pubkey: Pubkey,
    status: OrderStatus,
    payment_amount: u64,
    receipt: Option<Vec<u8>>,
}


#[derive(Clone, Debug, PartialEq)]
enum OrderStatus {
    Pending,
    PartiallyPaid,
    GoldBought,
    JewelryWorkStarted,
    Completed,
}


#[inline(never)]
fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    instruction_data: &[u8], 
) -> ProgramResult {
    
    let mut instruction_data = instruction_data;
    let instruction = instruction_data.read_u8().unwrap();
    instruction_data = &instruction_data[1..];

   
    let account_info_iter = &mut accounts.iter();

    
    let order_account = next_account_info(account_info_iter)?;

   
    if !order_account.owner.eq(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

   
    let mut account_data = order_account.data.borrow_mut();
    let order = match bincode::deserialize(&account_data) {
        Ok(order) => order,
        Err(_) => return Err(ProgramError::InvalidAccountData),
    };

     
    .into_bytes())

    
    match instruction {
        
        0 => {
           
            let customer_pubkey = Pubkey::new_from_array(instruction_data[..32].try_into().unwrap());
            instruction_data = &instruction_data[32..];
            let retailer_pubkey = Pubkey::new_from_array(instruction_data[..32].try_into().unwrap());

           
            let new_order = Order {
                id: order.id + 1,
                customer_pubkey,
                retailer_pubkey,
                status: OrderStatus::Pending,
                payment_amount: 0,
                receipt: None,
            };

            
            account_data.clear();
            let mut serialized_data = bincode::serialize(&new_order).unwrap();
            account_data.extend_from_slice(&serialized_data);

            
            Ok(new_order.id)
        }


        1 => {
           
            let receipt = instruction_data.to_vec();

            if !order.retailer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::Pending
            {
                return Err(ProgramError::InvalidAccountData);
            }

            
            let updated_order = Order {
                status: OrderStatus::PartiallyPaid,
                receipt: Some(receipt),
                ..order
            };

             account_data.clear();
            let mut serialized_data = bincode::serialize(&updated_product).unwrap();
            account_data.extend_from_slice(&serialized_data);

            
        2 => {
            
            let gold_price = instruction_data.read_u64::<LittleEndian>().unwrap();
            
            
            if !order.retailer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::PartiallyPaid
                || order.payment_amount != gold_price
            {
                return Err(ProgramError::InvalidAccountData);
            }
            
            
            let updated_order = Order {
                status: OrderStatus::GoldBought,
                ..order
            };
            
           
            account_data.clear();
            let mut serialized_data = bincode::serialize(&updated_order).unwrap();
            account_data.extend_from_slice(&serialized_data);

           
        3 => {
            
            if !order.retailer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::GoldBought
            {
                return Err(ProgramError::InvalidAccountData);
            }
        
           
            let updated_order = Order {
                status: OrderStatus::JewelryWorkStarted,
                ..order
            };
        
            
            account_data.clear();
            let mut serialized_data = bincode::serialize(&updated_order).unwrap();
            account_data.extend_from_slice(&serialized_data);

        4 => {
            
            if !order.retailer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::JewelryWorkStarted
            {
                return Err(ProgramError::InvalidAccountData);
            }
        
            
            let updated_order = Order {
                status: OrderStatus::Completed,
                ..order
            };
        
            
            account_data.clear();
            let mut serialized_data = bincode::serialize(&updated_order).unwrap();
            account_data.extend_from_slice(&serialized_data);

        5 => {
            
            let jewelry = instruction_data.to_vec();
        
            
            if !order.customer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::Completed
            {
                return Err(ProgramError::InvalidAccountData);
            }
        
            
            
             Ok(match order.status {
                OrderStatus::Pending => "Pending",
                OrderStatus::PartiallyPaid => "Partially paid",
                OrderStatus::GoldBought => "Gold bought",
                OrderStatus::JewelryWorkStarted => "Jewelry work started",
                OrderStatus::Completed => "Completed",
            }
            .to_string()
            .into_bytes())
            }
            
        
        }