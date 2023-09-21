import { retrieveToken } from "authHandler";

const client_id = process.env.CLIENT_ID;
const client_secret = process.env.CLIENT_SECRET;

const token = await retrieveToken(client_id, client_secret);