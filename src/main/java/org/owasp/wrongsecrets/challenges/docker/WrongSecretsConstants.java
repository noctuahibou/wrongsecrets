package org.owasp.wrongsecrets.challenges.docker;

import lombok.experimental.UtilityClass;

/** used for Challenges their secrets. */
@UtilityClass
public class WrongSecretsConstants {

  public static final String password = "DefaultLoginPasswordDoNotChange!";
  public static final String newKey = "mISydjhasklhgklhkkfD0En55Fq8FXbUfX720K8Vc6/aQYtkFmkp7ntsM=";

  mongosh "mongodb+srv://cluster0.0qe9ipy.mongodb.net/" --apiVersion 1 --username allankonar --password QDFH4HJ0zBuJlPLX
}

MongoClient mongoClient = MongoClients.create(
   MongoClientSettings.builder().applyConnectionString(new ConnectionString("mongodb+srv://allankonar:QDFH4HJ0zBuJlPLX@cluster0.0qe9ipy.mongodb.net:27017/testing?connectTimeoutMS=2000"))
      .applyToSocketSettings(builder ->
      builder.connectTimeout(5L, SECONDS))
      .build());

