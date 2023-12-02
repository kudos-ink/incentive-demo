import { Client } from '@notionhq/client'
import {
  QueryDatabaseParameters,
  QueryDatabaseResponse,
} from '@notionhq/client/build/src/api-endpoints'
const notion = new Client({ auth: process.env.NOTION_API_KEY })

export type NotionResponse = QueryDatabaseResponse | null

export async function notionQueryTable(
  notion: Client,
  query_object: QueryDatabaseParameters,
): Promise<NotionResponse> {
  // let query_object = { database_id, filter, filter_properties };

  try {
    let response = await notion.databases.query(query_object)
    return response
  } catch (err) {
    console.log(err)
    return null
  }
}
