"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const web3_js_1 = require("@solana/web3.js");
const spl_token_1 = require("@solana/spl-token");
const anchor_1 = require("@coral-xyz/anchor");
const nodewallet_1 = __importDefault(require("@coral-xyz/anchor/dist/cjs/nodewallet"));
const sdk_1 = require("@clockwork-xyz/sdk");
describe("spl-transfer", () => __awaiter(void 0, void 0, void 0, function* () {
    it("It transfers tokens every 10s", () => __awaiter(void 0, void 0, void 0, function* () {
        const connection = new web3_js_1.Connection("http://localhost:8899", "processed");
        const payer = keypairFromFile(require("os").homedir() + "/.config/solana/id.json");
        // Prepare clockworkProvider
        const provider = new anchor_1.AnchorProvider(connection, new nodewallet_1.default(payer), anchor_1.AnchorProvider.defaultOptions());
        const clockworkProvider = sdk_1.ClockworkProvider.fromAnchorProvider(provider);
        // Prepare dest
        const dest = web3_js_1.Keypair.generate().publicKey;
        const destAta = (yield (0, spl_token_1.getOrCreateAssociatedTokenAccount)(connection, payer, mint, // the address of the mint
        dest, false // is dest a pda?
        )).address;
        console.log(`dest: ${dest}, destAta: ${destAta}`);
    }));
}));
