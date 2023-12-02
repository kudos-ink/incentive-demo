import { Box, Text } from '@chakra-ui/react'
import { Client } from '@notionhq/client'
import {
  DatabaseObjectResponse,
  PartialDatabaseObjectResponse,
} from '@notionhq/client/build/src/api-endpoints'
import { GetServerSideProps } from 'next'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { notionQueryTable } from '../utils/notionQueryTable'
const notionClient = new Client({ auth: process.env.NOTION_API_KEY })

type FormData = {
  searchQuery: string
}

type PageProps = {
  defaultData: (PartialDatabaseObjectResponse | DatabaseObjectResponse)[]
}

export const getServerSideProps: GetServerSideProps = async () => {
  // Fetch default search results from Notion
  try {
    const response = await notionQueryTable(notionClient, {
      database_id: process.env.NOTION_ISSUES_DB_ID!,
      sorts: [
        {
          property: 'Opened Date',
          direction: 'descending',
        },
      ],
    })
    if (response) {
      let defaultData = response.results
      return { props: { defaultData } }
    } else {
      return { props: {} }
    }
  } catch (e) {
    console.log(e)
    return { props: {} }
  }
}

const IssuesPage = ({ defaultData }: PageProps) => {
  const [data, setData] = useState(defaultData)
  const { register, handleSubmit } = useForm<FormData>()

  //   const onSearch = async (formData: FormData) => {
  //     const searchQueryObject: QueryDatabaseParameters = {
  //       database_id: process.env.NOTION_ISSUES_DB_ID!,
  //     }
  //     const searchData = await notionQueryTable(notionClient, searchQueryObject)
  //     setData(searchData)
  //   }

  return (
    <Box>
      {data.map((item, index) => (
        <Box key={index} p={5} shadow="md" borderWidth="1px" mb={4}>
          {/* <Text>
            {
              item.properties['Project Name']['rollup']['array'][0]['rich_text'][0]['text'][
                'content'
              ]
            }
          </Text> */}
          <Text>{item.properties['Issue Title']['title'][0]['text']['content']}</Text>
          {/* <Link href={item.properties['Issue Link'].type['url']}>See it on GitHub</Link> */}
          {/* {JSON.stringify(item.properties, null, 2)} */}
          {/* Other item details */}
        </Box>
      ))}
    </Box>
  )
}

export default IssuesPage
