import { NextApiRequest, NextApiResponse } from 'next'
import Airtable from 'airtable'
import invalidBirthdate from '@/components/invalidBirthdate'

const airtable = new Airtable({ apiKey: process.env.AIRTABLE_KEY }).base(
  'appi7nULrVjE5uqtk'
)
const submissions = airtable('Submissions')

export default async function submit(
  req: NextApiRequest,
  res: NextApiResponse
) {
  console.log(req.body)
  if (
    !req.body.firstname ||
    !req.body.lastname ||
    !req.body.birthdate ||
    !req.body.email ||
    !req.body.address ||
    !req.body.city ||
    !req.body.state ||
    !req.body.zip ||
    !req.body.country ||
    !req.body.id ||
    !req.body.github ||
    !req.body.pr ||
    !req.body.demo ||
    !req.body.discovery
  )
    return res
      .status(401)
      .json({ error: 'Make sure you fill out all the fields!' })
  else if (invalidBirthdate(req.body.date))
    return res
      .status(401)
      .json({ error: 'You must be a high schooler or younger to submit here.' })

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
      if (err) return res.status(500).json({ err: err.toString() })
      return res.json(200)
    }
  )
}
