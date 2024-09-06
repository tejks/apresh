import { v4 as uuidv4 } from "uuid";
import { Server, StableBTreeMap, ic } from "azle";
import express from "express";
// import { Contract } from "azle/canisters/";

class Message {
  id: string;
  title: string;
  body: string;
  attachmentURL: string;
  createdAt: Date;
  updatedAt: Date | null;
}

const messagesStorage = StableBTreeMap<string, Message>(0);

export default Server(() => {
  const app = express();

  app.use(express.json());

  app.get("/settle", (req: any, res: any) => {
    //     const transferResult = ic.call(icpCanister.transfer, {
    //     args: [{
    //         memo: 0n,
    //         amount: {
    //             e8s: amount
    //         },
    //         fee: {
    //             e8s: transferFeeResponse.transfer_fee.e8s
    //         },
    //         from_subaccount: None,
    //         to: binaryAddressFromAddress(toAddress),
    //         created_at_time: None
    //     }]
    // });
    res.json(messagesStorage.values());
  });

  return app.listen();
});

function getCurrentDate() {
  const timestamp = new Number(ic.time());

  return new Date(timestamp.valueOf() / 1000_000);
}
