var express = require('express');
var router = express.Router();
import {getSSLHubRpcClient, Message} from "@farcaster/hub-nodejs";


const client = HUB_URL ? getSSLHubRpcClient(HUB_URL) : undefined;


/* GET home page. */
router.get('/', function(req, res, next) {
  if (req.body?.trustedData) {
    // has not voted
    res.render('index', { title: 'Express' });
  }
  else {
    res.render('results', { title: 'Express' });
  }
});

router.post('/', async function(req, res, next) {
  console.log(req)
  const frameMessage = Message.decode(Buffer.from(req.body?.trustedData?.messageBytes || '', 'hex'));
  const result = await client?.validateMessage(frameMessage);

  res.render('index', { title: 'Express' });
});





module.exports = router;
