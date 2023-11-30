import { FC } from 'react'
import { Contribution, ContributionCard } from './ContributionCard'

export interface ContributionsProps {
  list: Contribution[]
}
export const Contributions: FC<ContributionsProps> = ({ list }) => {
  return (
    <div>
      {list.map((x, idx) => (
        <ContributionCard key={idx} {...x} />
      ))}
    </div>
  )
}
