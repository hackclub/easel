import { NextApiRequest, NextApiResponse } from 'next'
import Airtable from 'airtable'

const airtable = new Airtable({ apiKey: process.env.AIRTABLE_KEY }).base(
  'appi7nULrVjE5uqtk'
)
const submissions = airtable('Submissions')

export default async function submit(
  req: NextApiRequest,
  res: NextApiResponse
) {
  console.log(req.body)
  submissions.create(
    {
      'First Name': req.body.firstname,
      'Last Name': req.body.lastname,
      'Birthday': req.body.birthdate,
      'Email': req.body.email,
      'Address (Line 1)': req.body.address,
      'Address (Line 2)': req.body.address2,
      'City': req.body.city,
      'State / Province': req.body.state,
      'ZIP / Postal Code': req.body.zip,
      'Country': req.body.country,
      'Student ID': req.body.id,
      'Code URL': req.body.pr,
      'Playable URL': req.body.demo,
      'GitHub Username': req.body.github,
      'How did you hear about this?': req.body.discovery,
      'What are we doing well?': req.body.compliments,
      'How can we improve?': req.body.improvements
    },
    (err, record) => {
      console.log(err, record)
      if (err) return res.status(500).json({ err: err.toString() })
      return res.json(200)
    }
  )
}
