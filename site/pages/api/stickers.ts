import { NextApiRequest, NextApiResponse } from 'next'
import Airtable from 'airtable'
import invalidBirthdate from '@/components/invalidBirthdate'

const airtable = new Airtable({ apiKey: process.env.AIRTABLE_KEY }).base(
  'appi7nULrVjE5uqtk'
)
const stickers = airtable('Stickers')

export default async function submit(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (
    !req.body.firstname ||
    !req.body.lastname ||
    !req.body.birthdate ||
    !req.body.address ||
    !req.body.email ||
    !req.body.city ||
    !req.body.state ||
    !req.body.zip ||
    !req.body.country
  )
    return res
      .status(401)
      .json({ error: 'Make sure you fill out all the fields!' })

  stickers.create(
    {
      'First Name': req.body.firstname,
      'Last Name': req.body.lastname,
      'Stickers': invalidBirthdate(req.body.birthdate) ? false : true,
      'Email': req.body.email,
      'Address (Line 1)': req.body.address,
      'Address (Line 2)': req.body.address2,
      'City': req.body.city,
      'State / Province': req.body.state,
      'ZIP / Postal Code': req.body.zip,
      'Country': req.body.country
    },
    (err, record) => {
      if (err) return res.status(500).json({ err: err.toString() })
      return res.json(200)
    }
  )
}
