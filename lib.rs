
solana_program::declare_id!("your_program_id_here");

// Define the order struct
#[derive(Clone, Debug, PartialEq)]
struct Order {
    id: u64,
    customer_pubkey: Pubkey,
    retailer_pubkey: Pubkey,
    status: OrderStatus,
    payment_amount: u64,
    receipt: Option<Vec<u8>>,
}

// Define the order status enum
#[derive(Clone, Debug, PartialEq)]
enum OrderStatus {
    Pending,
    PartiallyPaid,
    GoldBought,
    JewelryWorkStarted,
    Completed,
}

// Define the entrypoint function
#[inline(never)]
fn process_instruction(
    program_id: &Pubkey, // the ID of this program
    accounts: &[AccountInfo], // accounts involved in the transaction
    instruction_data: &[u8], // instruction data passed to the program
) -> ProgramResult {
    // Deserialize the instruction data
    let mut instruction_data = instruction_data;
    let instruction = instruction_data.read_u8().unwrap();
    instruction_data = &instruction_data[1..];

    // Get the accounts involved in the transaction
    let account_info_iter = &mut accounts.iter();

    // Get the order account
    let order_account = next_account_info(account_info_iter)?;

    // Check that the order account is a valid account
    if !order_account.owner.eq(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the order from the account data
    let mut account_data = order_account.data.borrow_mut();
    let order = match bincode::deserialize(&account_data) {
        Ok(order) => order,
        Err(_) => return Err(ProgramError::InvalidAccountData),
    };

     
    .into_bytes())

    // Perform the requested action
    match instruction {
        // Customer places an order
        0 => {
            // Deserialize the customer and retailer public keys from the instruction data
            let customer_pubkey = Pubkey::new_from_array(instruction_data[..32].try_into().unwrap());
            instruction_data = &instruction_data[32..];
            let retailer_pubkey = Pubkey::new_from_array(instruction_data[..32].try_into().unwrap());

            // Create the new order
            let new_order = Order {
                id: order.id + 1,
                customer_pubkey,
                retailer_pubkey,
                status: OrderStatus::Pending,
                payment_amount: 0,
                receipt: None,
            };

            // Serialize the new order and store it in the account data
            account_data.clear();
            let mut serialized_data = bincode::serialize(&new_order).unwrap();
            account_data.extend_from_slice(&serialized_data);

            // Return the new order ID
            Ok(new_order.id)
        }

        // Retailer acknowledges the order with a receipt
        1 => {
            // Deserialize the receipt from the instruction data
            let receipt = instruction_data.to_vec();

            // Check that the sender is the retailer and the order is in the Pending state
            if !order.retailer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::Pending
            {
                return Err(ProgramError::InvalidAccountData);
            }

            // Update the order's receipt and status fields
            let updated_order = Order {
                status: OrderStatus::PartiallyPaid,
                receipt: Some(receipt),
                ..order
            };

             account_data.clear();
            let mut serialized_data = bincode::serialize(&updated_product).unwrap();
            account_data.extend_from_slice(&serialized_data);

            
        2 => {
            // Deserialize the gold price from the instruction data
            let gold_price = instruction_data.read_u64::<LittleEndian>().unwrap();
            
            // Check that the sender is the retailer, the order is in the PartiallyPaid state, and the payment amount matches the gold price
            if !order.retailer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::PartiallyPaid
                || order.payment_amount != gold_price
            {
                return Err(ProgramError::InvalidAccountData);
            }
            
            // Update the order's status field to GoldBought
            let updated_order = Order {
                status: OrderStatus::GoldBought,
                ..order
            };
            
            // Serialize the updated order and store it in the order account
            account_data.clear();
            let mut serialized_data = bincode::serialize(&updated_order).unwrap();
            account_data.extend_from_slice(&serialized_data);

            // Retailer starts the jewelry work
        3 => {
            // Check that the sender is the retailer and the order is in the GoldBought state
            if !order.retailer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::GoldBought
            {
                return Err(ProgramError::InvalidAccountData);
            }
        
            // Update the order's status field to JewelryWorkStarted
            let updated_order = Order {
                status: OrderStatus::JewelryWorkStarted,
                ..order
            };
        
            // Serialize the updated order and store it in the account data
            account_data.clear();
            let mut serialized_data = bincode::serialize(&updated_order).unwrap();
            account_data.extend_from_slice(&serialized_data);

        4 => {
            // Check that the sender is the retailer and the order is in the JewelryWorkStarted state
            if !order.retailer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::JewelryWorkStarted
            {
                return Err(ProgramError::InvalidAccountData);
            }
        
            // Update the order's status field to Completed
            let updated_order = Order {
                status: OrderStatus::Completed,
                ..order
            };
        
            // Serialize the updated order and store it in the order account
            account_data.clear();
            let mut serialized_data = bincode::serialize(&updated_order).unwrap();
            account_data.extend_from_slice(&serialized_data);

        5 => {
            // Deserialize the jewelry from the instruction data
            let jewelry = instruction_data.to_vec();
        
            // Check that the sender is the customer and the order is in the Completed state
            if !order.customer_pubkey.eq(account_info_iter.next().unwrap().key)
                || order.status != OrderStatus::Completed
            {
                return Err(ProgramError::InvalidAccountData);
            }
        
            // Return a success message
            
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