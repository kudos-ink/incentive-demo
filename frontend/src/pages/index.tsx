import { Contributions } from '@/components/contributions/ContributionList'
import { HomePageTitle } from '@/components/home/HomePageTitle'
import { CenterBody } from '@/components/layout/CenterBody'
import { ChainInfo } from '@/components/web3/ChainInfo'
import { ConnectButton } from '@/components/web3/ConnectButton'
import { ContributionStatus } from '@/components/web3/ContributionStatus'
import { MOCK_CONTRIBUTIONS } from '@/utils/constants'
import { useInkathon } from '@scio-labs/use-inkathon'
import type { NextPage } from 'next'
import { useEffect } from 'react'
import { toast } from 'react-hot-toast'
import 'twin.macro'

const HomePage: NextPage = () => {
  // Display `useInkathon` error messages (optional)
  const { error } = useInkathon()
  useEffect(() => {
    if (!error) return
    toast.error(error.message)
  }, [error])

  return (
    <>
      {/* Top Bar */}
      {/* <HomeTopBar /> */}

      <h3 tw="mb-4 flex cursor-pointer items-center gap-4 rounded-3xl py-1.5 px-3.5 font-bold text-2xl">
        Available contributions
      </h3>
      <Contributions list={MOCK_CONTRIBUTIONS} />
    </>
  )
}

export default HomePage
