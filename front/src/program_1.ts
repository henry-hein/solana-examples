import WertWidget from '@wert-io/widget-initializer';
import type { Options } from '@wert-io/widget-initializer/types';
import { signSmartContractData } from '@wert-io/widget-sc-signer';
import { v4 as uuidv4 } from 'uuid';

import { Buffer } from 'buffer/';

import {
  PublicKey,
  SystemProgram,
  PublicKeyInitData
} from "@solana/web3.js";

window.Buffer = Buffer; // needed to use `signSmartContractData` in browser

function createDepositInstruction(userAddress: PublicKeyInitData, amount: number) {
  // --- Constants and Program ID ---
  const PRESALE_PROGRAM_ID = new PublicKey("C3tQLiwnKZjEeQXx5bmVgHSYorBCCX4aCEnreL648aJn");

  const userPublicKey = new PublicKey(userAddress);

  // get from /target/idl/presale_program.json
  const discriminator = Buffer.from([108, 81, 78, 117, 125, 155, 56, 200]);

  // 8 bytes: amount as u64 (Number to 8-byte buffer, little-endian)
  const amountBytes = Buffer.alloc(8);
        //@ts-expect-error
        amountBytes.writeBigUInt64LE(BigInt(amount), 0); 

  const instructionData = Buffer.concat([
    discriminator,
    amountBytes,
  ]);

  const signer = new PublicKey("BGCSawehjnxUDciqRCPfrXqzKvBeiTSe3mEtvTFC5d9q"); 

  const instruction = {
    program_id: PRESALE_PROGRAM_ID,
    accounts: [
      // 0. user 
      { address: userPublicKey, is_signer: false, is_writable: true },
      // 1. signer (mutable, signer) 
      { address: signer, is_signer: true, is_writable: true },
      // 2. system_program (read-only)
      { address: SystemProgram.programId, is_signer: false, is_writable: false },
    ],
    data: instructionData,
  };

  return instruction;
}

// --- 2. Wert Widget Configuration and Initialization ---

const program_id = 'C3tQLiwnKZjEeQXx5bmVgHSYorBCCX4aCEnreL648aJn';
const sol_amount = 0.02;
const user_address = '2cU8waWn5fdwvr1fcbbD2jHV4qbRNBsSj4xfshxbf3pL';
const privateKey =
  '0x57466afb5491ee372b3b30d82ef7e7a0583c9e36aef0f02435bd164fe172b1d3';

const instruction = createDepositInstruction(user_address, sol_amount*1000000000);

const wertInstructionJson = {
  program_id: instruction.program_id.toBase58(),
  accounts: instruction.accounts.map(k => ({
    address: k.address.toBase58(),
    is_signer: k.is_signer,
    is_writable: k.is_writable,
  })),
  data: instruction.data.toString("hex"),
};

const scInputData = Buffer.from(JSON.stringify(wertInstructionJson)).toString('hex');

const signedData = signSmartContractData(
  {
    address: user_address,
    commodity: 'sol',
    network: 'solana',
    commodity_amount: sol_amount,
    sc_address: program_id,
    sc_input_data: scInputData,
  },
  privateKey
);

const my_click_id = uuidv4();

const otherWidgetOptions: Options = {
  partner_id: '01JSE6XPWNW07YFYMV30EP6PF4',
  click_id: my_click_id,
  origin: 'https://sandbox.wert.io', // this option needed only for this example to work
  phone: "+12012000205",
};


const wertWidget = new WertWidget({
  ...signedData,
  ...otherWidgetOptions,
});

document.addEventListener('DOMContentLoaded', () => {
  const button = document.getElementById('widget-open');
  if (button) {
    button.addEventListener('click', () => {
      wertWidget.open();
    });
  }
});
