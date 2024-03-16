var express = require('express');
const { getSSLHubRpcClient, Message } = require("@farcaster/hub-nodejs");
var router = express.Router();

const HUB_URL = process.env['HUB_URL']
const client = HUB_URL ? getSSLHubRpcClient(HUB_URL) : undefined;


/* GET home page. */
router.get('/', function(req, res, next) {
    res.render('index', { title: 'Express' });
});

router.post('/', async function(req, res, next) {
  console.log(req.body)

  const frameMessage = Message.decode(Buffer.from(req.body?.trustedData?.messageBytes || '', 'hex'));
  const result = await client?.validateMessage(frameMessage);

  res.render('results', { title: 'Express' });
});





module.exports = router;
