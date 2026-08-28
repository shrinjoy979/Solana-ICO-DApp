![1758127560502](https://github.com/user-attachments/assets/ca0dc7e9-adf7-4716-8f8f-b5cf8eebc614)
![1758127560681](https://github.com/user-attachments/assets/5c9237a5-197e-45a0-b809-c1cd65f403ed)
![1758127560667](https://github.com/user-attachments/assets/21076eb7-ec08-48d3-a7b5-121febcb59a3)
![1758127560600](https://github.com/user-attachments/assets/6e9657e7-ccb3-4aec-bdcc-3cf6bb48c533)

# Solana ICO DApp

A decentralized ICO (Initial Coin Offering) platform built on the Solana blockchain.
Users can initialize the ICO, purchase tokens using SOL, and track token distribution in real time.

**Live Demo:** [solana-ico-d-app-ten.vercel.app](https://solana-ico-d-app-ten.vercel.app)

---

## User Dashboard

- View your token balance
- Buy tokens with SOL
- Real-time cost calculation
- Network fee breakdown
- Automatic balance updates after purchase

---

## Features

- ICO Initialization (Admin)
- Secure token purchasing
- Live supply tracking
- On-chain transaction verification

---

## Tech Stack

- **Frontend:** Next.js 13, React 18
- **Smart Contract:** Anchor (Rust) — see `/Contract`
- **Blockchain:** Solana Web3.js
- **Wallet Integration:** Solana Wallet Adapter
- **Token Standard:** SPL Token
- **Styling:** Tailwind CSS

---

## Project Structure
Solana-ICO-DApp/
├── Contract/ # Anchor smart contract (Rust) for the ICO program
├── lib/ # Shared utility functions and helpers
├── pages/ # Next.js pages / routes
├── styles/ # Global and component styles
├── .env.example # Sample environment variables
└── package.json


---

## Setup Project

1. Clone the repository:
```bash
   git clone https://github.com/shrinjoy979/Solana-ICO-DApp.git
   cd Solana-ICO-DApp
```

2. Install dependencies:
```bash
   npm install
```

3. Create a `.env` file in the project root (based on `.env.example`) and add your deployed program details:
```env
   NEXT_PUBLIC_PROGRAM_ID=your_deployed_ico_program_id
   NEXT_PUBLIC_ICO_MINT=your_ico_token_mint_address
```

4. (If deploying your own contract) Build and deploy the Anchor program in `/Contract` to Devnet, then use the resulting Program ID and Token Mint address above.

5. Run the development server:
```bash
   npm run dev
```

6. Open [http://localhost:3000](http://localhost:3000) in your browser.

---

## Available Scripts

| Command         | Description                          |
|-----------------|---------------------------------------|
| `npm run dev`   | Start the local development server    |
| `npm run build` | Build the app for production          |
| `npm run start` | Run the production build              |
| `npm run lint`  | Run ESLint checks                     |

---

## Admin Flow

1. Connect an admin wallet.
2. Initialize the ICO (sets total supply, price, and mint authority).
3. Monitor live token sales and remaining supply from the dashboard.

## User Flow

1. Connect your Solana wallet (e.g. Phantom, Solflare).
2. Enter the amount of SOL you want to spend.
3. Review the real-time token cost and network fee breakdown.
4. Confirm the transaction and see your balance update automatically.

---

## Contributing

Contributions, issues, and feature requests are welcome. Feel free to check the [issues page](https://github.com/shrinjoy979/Solana-ICO-DApp/issues).

## License

This project currently has no license file specified. Add a `LICENSE` file if you intend to open-source this under a specific license (e.g., MIT).
