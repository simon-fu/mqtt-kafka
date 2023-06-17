
CREATE TABLE `uplink_messages` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `app_id` varchar(20) NOT NULL,
  `create_time` datetime(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  `msg_id` bigint unsigned NOT NULL,
  `client_id` varchar(200) NOT NULL,
  `conn_id` varchar(20) NOT NULL,
  `topic` varchar(64) NOT NULL,
  `qos` tinyint NOT NULL,
  `publish_time` datetime NOT NULL,
  `payload` blob NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `mqtt_messages_msg_id_UN` (`msg_id`),
  KEY `app_id_topic_pub_time_IDX` (`app_id`, `topic`, `publish_time`) USING BTREE,
  KEY `app_id_client_id_IDX` (`app_id`,`client_id`) USING BTREE,
  KEY `create_time_IDX` (`create_time`) USING BTREE,
  KEY `conn_id_IDX` (`conn_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- CREATE TABLE IF NOT EXISTS mqtt_messages
-- (
--     id bigint(20) NOT NULL, -- AUTO_INCREMENT,
--     app_id varchar(20) NOT NULL,
--     create_time datetime(3) DEFAULT CURRENT_TIMESTAMP(3),
--     msg_id bigint(20) NOT NULL,
--     client_id varchar(20) NOT NULL,
--     conn_id varchar(20) NOT NULL,
--     qos: tinyint NOT NULL,
--     publish_time datetime(3) DEFAULT CURRENT_TIMESTAMP(3),
--     payload blob NOT NULL,
--     PRIMARY KEY (`id`),
--     UNIQUE KEY `{mqtt_messages_msg_id_UN` (`msg_id`),
--     KEY `idx-mqtt_messages-app_id` (`app_id`),
--     KEY `idx-mqtt_messages-app_id_client_id` (`app_id`, `client_id`),
--     KEY `idx-mqtt_messages-app_id_pub_time` (`app_id`, `publish_time`),
--     KEY `idx-mqtt_messages-create_time` (`create_time`),
--     KEY `idx-mqtt_messages-conn_id` (`conn_id`),
    
    
--     -- PRIMARY KEY (`frag_type`,`frag_id`,`ch_id`)
--     -- `name` varchar(20) COLLATE utf8_unicode_ci NOT NULL,

--     -- id          INTEGER PRIMARY KEY NOT NULL,
--     -- msg_id      INTEGER             NOT NULL,
--     -- done        BOOLEAN             NOT NULL DEFAULT 0
-- );
