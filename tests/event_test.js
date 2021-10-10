const anchor = require('@project-serum/anchor');

describe('event_test', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.EventTest;
    let tx = await program.rpc.initializeA();
    console.log("Your transaction signature", tx);
     tx = await program.rpc.initializeB();
    console.log("Your transaction signature", tx);
  });
});
