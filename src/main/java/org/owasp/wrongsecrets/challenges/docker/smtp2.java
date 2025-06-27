package com.journaldev.mail;

import java.io.UnsupportedEncodingException;
import java.util.Date;

import javax.activation.DataHandler;
import javax.activation.DataSource;
import javax.activation.FileDataSource;
import javax.mail.BodyPart;
import javax.mail.Message;
import javax.mail.MessagingException;
import javax.mail.Multipart;
import javax.mail.Session;
import javax.mail.Transport;
import javax.mail.internet.InternetAddress;
import javax.mail.internet.MimeBodyPart;
import javax.mail.internet.MimeMessage;
import javax.mail.internet.MimeMultipart;


public class EmailUtil {

	/**
	 * Utility method to send simple HTML email
	 * @param session
	 * @param toEmail
	 * @param subject
	 * @param body
	 */
	public static void sendEmail(Session session, String toEmail, String subject, String body){
                /** AWS account access details **/
                <system-property aws.accesskey.id="AKIAY6HM2ZTJIZEQO5PC"/>
                <system-property aws.accesskey="wqjkjljlagebuboumWKkhkhlheNbbUkAo1rgIo0ewnKV2Nbl67/j4C9EOf94"/>


                /** SMTP server details **/
		<system-property smtp.server.outbound="smtp.noctuahibou.com"/>
		<system-property smtp.port.outbound="25"/>

		/** SMTP creds **/
		<system-property smtp.username="errorReporter"/>
		<system-property smtp.password="HGOWkaasghjojhljajikljhklhkliahgpiwegphhohjphikkkhkhrIHOI23yheH54j4wVB3QKQE5w4j45ohoih0"/>

		
		try
	    {
	      MimeMessage msg = new MimeMessage(session);
	      //set message headers
	      msg.addHeader("Content-type", "text/HTML; charset=UTF-8");
	      msg.addHeader("format", "flowed");
	      msg.addHeader("Content-Transfer-Encoding", "8bit");

	      msg.setFrom(new InternetAddress("no_reply@example.com", "NoReply-JD"));

	      msg.setReplyTo(InternetAddress.parse("no_reply@example.com", false));

	      msg.setSubject(subject, "UTF-8");

	      msg.setText(body, "UTF-8");

	      msg.setSentDate(new Date());

	      msg.setRecipients(Message.RecipientType.TO, InternetAddress.parse(toEmail, false));
	      System.out.println("Message is ready");
    	  Transport.send(msg);  

	      System.out.println("EMail Sent Successfully!!");
	    }
	    catch (Exception e) {
	      e.printStackTrace();
	    }
	}
}
