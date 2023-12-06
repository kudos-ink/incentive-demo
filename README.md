# Kudos Portal

## About

Kudos Portal is a platform that onboards new contributors to the Substrate, Polkadot & Kusama ecosytem. It serves as an ideal hub for these communities to showcase their projects and encourage learning and discovery among developers. This solution integrates seamlessly with your GitHub projects, benefiting from a trustless, automated reward system that allows you to tag issues with custom incentives. Be it bounties, POAPs, or other utilities, the choice is yours. Developers are attracted to tasks matching their skill sets and interests, and upon completion, our trustless system ensures they are rewarded without any middlemen. The customization stems from our ink! smart contract templates and GitHub workflows, which automate the process.

## Background

After leaving the Polkadot Blockchain Academy, alumni find it challenging to locate GitHub issues to continue learning and contributing. This problem, which we have personally encountered, is a common struggle in the open-source software ecosystem. Additionally, incentivizing contributions could attract new developers to build on the ecosystem. However, no solution providing a trustless environment where contributors can receive their dues without intermediaries exists. Be it for reward distribution or for monitoring and reputation.

We showcased a proof of concept (POC) of this project at the [Polkadot ink! hackathon](https://www.encode.club/polkadot-ink-hackathon), winning the main prize and additional sponsor awards two weeks ago. Here is our 3-minute [finale pitch](https://www.youtube.com/watch?v=zw07lCW639w). Today, we are maintaining this momentum by participating in the [Encode accelerator program](https://www.encode.club/encode-polkadot-accelerator-2023).

## How it works?

The contribution flow relies on two main characters:
- An `organization` (or `project maintainer`) calling for contributors to address certain Github issues. 
- `Aspiring contributors`

### Step 0: Prerequisities

- The organization connects to the Kudos Portal and marks some of its existing Github project as `Looking for contributors`.
- Optionnally the organization can tag some project's issues with custom incentives from the portal. It instantiates a **reward contract** based on available contract templates from our [Kudos Factory](https://github.com/kudos-ink/contracts) or specifying a custom one. The custom contract needs to implement the interface defined by the [workflow trait](https://github.com/kudos-ink/contracts/blob/main/contracts/src/traits/workflow.rs). The first time a **reward contract** is added to a project contribution, a pull request (PR) is automatically opened on the Github repository of the project adding the corresponding workflow.

### Step 1: Aspiring Contributor

- An aspiring finds an available contribution on the Kudos Portal. After being assigned it, he starts working on it.
- The contributor opens a PR to resolve the issue specifying its address.

### Step 2: Approval

- The organization reviews, approves, and merges the PR, thereby closing the issue.
- The **reward workflow** is triggered and calls `approve` on the **reward contract** with the given issue #ID and the contributor address.

### Step 3: Claim

- The contributor claims its reward from the Kudos Portal by using the `claim` method on the **reward contract** with the issue #ID. This action triggers the custom reward flow specified in the contract (e.g. a single bounty transfer, an NFT minting, ..)

## Architecture

### Customization aspects

This approach offers a high level of customization. The `claim` method, the final step in the process, allows for a wide range of reward mechanisms, making it highly adaptable to the specific needs and preferences of the organization and contributors. This flexibility empowers projects to implement diverse and tailored reward structures, whether it's a straightforward balance transfer, the issuance of non-fungible tokens (NFTs), or other creative and unique reward systems, ensuring that the Kudos Ink! platform can seamlessly accommodate a variety of open-source project needs and preferences. 

Kudos Ink! provides some templates ready to use as [reward workflow](https://github.com/kudos-ink/workflow-example/blob/main/.github/workflows/issue-closed.yml) with dedicated Github actions and [reward contract](https://github.com/kudos-ink/contracts/blob/main/contracts/src/token/single-token/lib.rs)

## Tech Stack

### Open brush support

Kudos Ink! supports [OpenBrush](https://github.com/Brushfam/openbrush-contracts). The `approve` method extends the [Ownable](https://learn.brushfam.io/docs/OpenBrush/smart-contracts/ownable) contract from OpenBrush.

### ink!athon Boilerplate

This project uses the [ink!athon Boilerplate](https://github.com/scio-labs/inkathon) full-stack dApp boilerplate for ink! smart contracts with an integrated frontend.

## Use this demo

In this demo, a contribution refers to "opening an issue in this repository". A proof of contribution will be registered in the demo contract.
Below is the detailed workflow:

![Kudos Ink! Demo Flow](kudos-ink_demo.png)

#### Steps:

1. Connect and spot an available contribution on the Kudos Portal.

   This action marks your intention to contribute to a project. In this case, to test the demo.

2. Complete the expected work. For this demo, you are required to **open an issue in this repository**.

3. Once the issue is opened, a proof of contribution will be automatically registered into the demo smart contract. You will be notified in an issue comment with a link to the proof.

4. Finally, check your contribution status, marked as complete, in the Demo UI app.

## Next steps

The Kudos team has dedicated itself to developing within the Polkadot ecosystem over recent months. Our aim is for the community to recognize the significance of our upcoming project. We eagerly welcome feedback and are readily available to address any inquiries. Looking forward to connecting soon!
