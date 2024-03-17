# Official Readme
1. This is the infrastrucure for a community app which uses farcaster frames.
2. Each community has it's own vault (SAFE Smart Account) in which they hold their funds. <- Example instance of Vault Contract exists on the Arbitrum network at address: arb:0x26Ef58BBBB086D6c2DEdebfAD6FFd10c80Ca70Aa 
3. When a user signs up, we create a SAFE for their wallet, which is controlled through account abstraction using Magic.
4. Voting to spend the funds in the volt happens through interacting with a FRAME on FARCASTER.
    - In FRAME_NODE is the code which runs a node.js server which hosts all frames (currently just 1 example frame)
    - When a user votes (clicks reject/approve) on the frame, the server maps his fid to an ethereum address and records it and his vote direction in a Poll (an ARBITRUM STYLUS contract). 
    - That Poll contract which is deployed by a SAFE_MODULE is responsible for triggering a function in the safe_module to release the funds to the Poll proposer.

# CommunityCaster
- Account abstraction using Magic to creating an acocunt for the app.
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