use crate::lightning::is_valid_invoice;
use crate::util::send_order_id_cmd;
use anyhow::Result;
use lnurl::lightning_address::LightningAddress;
use mostro_core::message::{Action, Content, Message};
use nostr_sdk::prelude::*;
use std::str::FromStr;
use uuid::Uuid;

pub async fn execute_add_invoice(
    order_id: &Uuid,
    invoice: &str,
    my_key: &Keys,
    mostro_key: PublicKey,
    client: &Client,
) -> Result<()> {
    println!(
        "Sending a lightning invoice {} to mostro pubId {}",
        order_id, mostro_key
    );
    let mut content = None;
    // Check invoice string
    let ln_addr = LightningAddress::from_str(invoice);
    if ln_addr.is_ok() {
        content = Some(Content::PaymentRequest(None, invoice.to_string(), None));
    } else {
        match is_valid_invoice(invoice) {
            Ok(i) => content = Some(Content::PaymentRequest(None, i.to_string(), None)),
            Err(e) => println!("{}", e),
        }
    }
    // Create AddInvoice message
    let add_invoice_message =
        Message::new_order(None, Some(*order_id), Action::AddInvoice, content)
            .as_json()
            .unwrap();

    send_order_id_cmd(client, my_key, mostro_key, add_invoice_message, true, false).await?;

    Ok(())
}
