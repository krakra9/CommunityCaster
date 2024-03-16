# CommunityCaster
- Account Abstraction for creating an acocunt for the app
- User can join a community -> They will pay a monthly fee
- Monthy fee is pooled together and only accesaable by community vote
- Person can post a porposal
- all is done through farcaster frames and MACI infactstrucutre for avoiding colusion
- SmartContract logic is hosted on Arbitrum Nova(think it's better than one for our usecase because it's ceaper)

#### Sign Up
User signes up (SAFE makes them a Web3 Account using Pimloco and permisonless.js).
Create a farcaster account for users(somehow associate with SAFE smart account??)

#### Join/Create Community
Community is a farcaster channel. (how does Warpcast do this with their channels?)
To join the channel you need to set up monthly paymetnes. <- SAFE has a way of making this web2 friendly

#### Propose an event
- User writes a proposal and suggests a price for doing what they porpose (eg: Organise a webinar with someone famous in the field).
- Users see proposal as a farcaster frame and can vote on it (MACI infrastructure in the backend) 
    - For the proposal to go through: 
        (Monthly Fee) * (Approvers-Rejectors) >= proposal price;
- Frames interact with a smartcontract deployed on Arbitrum 




# Latest Thinking
1. When a button is pressed in the frame, the frame server takes the button id and the FID.
2. It maps the button to an approve or reject and fid to the ETH wallet through the fir registry.
3. Meanwhile, in an Arbitrum smart contract we wrtie to a PollMap(address -> Choice);
4. Wait until Specified Decision blockHeight and the NODE signs a multisig transaction for moving funds from vault to propser.
5. Proposer Makes a call to Vault contract and makes IT provide the second signitrue for moving the funds to the proposers account.