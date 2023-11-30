import { NextPage } from 'next'
import { ContributionStatus } from '@/components/web3/ContributionStatus'
import { FiChevronLeft } from 'react-icons/fi'

const CheckPage: NextPage = () => {
  return (
    <div tw="mt-10 flex w-full flex-wrap items-start justify-center gap-4">
      {/* Check Contribution status */}
      <ContributionStatus />
    </div>
  )
}

export default CheckPage
